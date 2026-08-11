//! One leg of the zigzag: where it started, how far it has got, and how much
//! it has given back.

use chrono::{DateTime, Utc};
use nsc_core::candle::Candle;
use nsc_core::price::{Price, PriceDistance};
use nsc_core::swing::{Swing, SwingKind};

use super::direction::{is_behind, is_beyond, sides};
use super::memory::RunMemory;
use super::run::{Extreme, share, span};
use super::step::Step;
use crate::config::SwingSettings;
use crate::error::TaError;

/// A move in one direction, still going.
///
/// `hunting` is what would end it: on the way up we are hunting a high.
#[derive(Debug, Clone)]
pub(super) struct Leg {
    hunting: SwingKind,

    /// Where this run started. A confirmed swing, once there has been one.
    anchor: Extreme,

    /// The furthest the run has got so far — the candidate swing.
    extreme: Extreme,

    /// The furthest price has come back since the extreme.
    retrace: Extreme,
}

impl Leg {
    pub fn new(hunting: SwingKind, anchor: Extreme, extreme: Extreme, retrace: Extreme) -> Self {
        Self {
            hunting,
            anchor,
            extreme,
            retrace,
        }
    }

    pub fn take(
        &mut self,
        candle: &Candle,
        settings: &SwingSettings,
        memory: &RunMemory,
    ) -> Result<Step, TaError> {
        let now = candle.open_time();
        let (forward, backward) = sides(self.hunting, candle);

        // The run has been undone completely — price is past where it began.
        // Nothing here was structure, so the run simply restarts from here.
        if is_behind(self.hunting, backward, self.anchor.price) {
            self.anchor = Extreme::new(backward, now);
            self.extreme = Extreme::new(forward, now);
            self.retrace = Extreme::new(backward, now);
            return Ok(Step::Continue);
        }

        if let Some(step) = self.look(candle, settings, memory)? {
            return Ok(step);
        }

        self.absorb(candle);

        Ok(Step::Continue)
    }

    /// Has anything proved itself on this candle? Changes nothing.
    ///
    /// Kept separate from [`take`](Self::take) because the seed asks the same
    /// question of a leg it rebuilds from scratch each candle.
    pub(super) fn look(
        &self,
        candle: &Candle,
        settings: &SwingSettings,
        memory: &RunMemory,
    ) -> Result<Option<Step>, TaError> {
        let now = candle.open_time();
        let (forward, backward) = sides(self.hunting, candle);

        // Price has taken the old extreme out. If it had already given back
        // enough on the way, that pause was a real pullback and both ends of
        // it are swings.
        if is_beyond(self.hunting, forward, self.extreme.price) {
            return self.shallow_route(forward, backward, now, settings, memory);
        }

        // Otherwise the question is whether this candle deepens the give-back
        // far enough to prove the extreme on its own.
        let mut deepened = self.clone();
        deepened.absorb(candle);

        deepened.depth_route(forward, now, settings, memory)
    }

    /// Takes the candle in without deciding anything.
    fn absorb(&mut self, candle: &Candle) {
        let now = candle.open_time();
        let (forward, backward) = sides(self.hunting, candle);

        if is_beyond(self.hunting, forward, self.extreme.price) {
            self.extreme = Extreme::new(forward, now);
            self.retrace = Extreme::new(backward, now);
        } else if is_behind(self.hunting, backward, self.retrace.price) {
            self.retrace = Extreme::new(backward, now);
        }
    }

    /// Enough has been given back for the extreme to prove itself on its own.
    fn depth_route(
        &self,
        forward: Price,
        now: DateTime<Utc>,
        settings: &SwingSettings,
        memory: &RunMemory,
    ) -> Result<Option<Step>, TaError> {
        // A candle cannot confirm a peak it made itself. The peak is only
        // knowable once a later candle has come back off it.
        if now <= self.extreme.bar_time || !self.given_back_enough(settings.confirm_retracement) {
            return Ok(None);
        }

        let run = self.run();
        if !memory.allows(run) {
            return Ok(None);
        }

        let swing = Swing::new(self.hunting, self.extreme.bar_time, now, self.extreme.price)?;

        Ok(Some(Step::Confirmed {
            swings: vec![swing],
            next: Leg::new(
                self.hunting.opposite(),
                self.extreme,
                self.retrace,
                Extreme::new(forward, now),
            ),
            run,
        }))
    }

    /// A shallower pause counts too, now that price has taken the extreme out.
    fn shallow_route(
        &self,
        forward: Price,
        backward: Price,
        now: DateTime<Utc>,
        settings: &SwingSettings,
        memory: &RunMemory,
    ) -> Result<Option<Step>, TaError> {
        let a_real_pause =
            self.retrace.bar_time > self.extreme.bar_time && now > self.retrace.bar_time;

        if !a_real_pause || !self.given_back_enough(settings.shallow_retracement) {
            return Ok(None);
        }

        let run = self.run();
        if !memory.allows(run) {
            return Ok(None);
        }

        let end_of_run = Swing::new(self.hunting, self.extreme.bar_time, now, self.extreme.price)?;
        let end_of_pause = Swing::new(
            self.hunting.opposite(),
            self.retrace.bar_time,
            now,
            self.retrace.price,
        )?;

        Ok(Some(Step::Confirmed {
            swings: vec![end_of_run, end_of_pause],
            // The pause was the start of the next run, and it is still going
            // the same way.
            next: Leg::new(
                self.hunting,
                self.retrace,
                Extreme::new(forward, now),
                Extreme::new(backward, now),
            ),
            run,
        }))
    }

    fn run(&self) -> PriceDistance {
        span(self.anchor.price, self.extreme.price)
    }

    fn given_back_enough(&self, needed: f64) -> bool {
        let given_back = span(self.extreme.price, self.retrace.price);

        share(given_back, self.run()).is_some_and(|share| share >= needed)
    }
}
