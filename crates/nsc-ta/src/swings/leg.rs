//! One leg of the zigzag: where it started, how far it has got, and how much
//! it has given back.

use chrono::{DateTime, Utc};
use nsc_core::candle::Bar;
use nsc_core::swing::{Swing, SwingError, SwingKind};
use rust_decimal::Decimal;

use super::Rules;
use super::extreme::{Extreme, share, span};
use super::facing::{is_behind, is_beyond, sides};
use super::memory::RunMemory;
use super::step::Step;

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
    pub(super) fn new(
        hunting: SwingKind,
        anchor: Extreme,
        extreme: Extreme,
        retrace: Extreme,
    ) -> Self {
        Leg {
            hunting,
            anchor,
            extreme,
            retrace,
        }
    }

    pub(super) fn take(
        &mut self,
        bar: &Bar,
        now: DateTime<Utc>,
        rules: &Rules,
        memory: &RunMemory,
    ) -> Result<Step, SwingError> {
        let (forward, backward) = sides(self.hunting, bar);

        // **Ask what this candle proved BEFORE anything else.** A candle that
        // dives straight past the start of the run has given back more than
        // all of it, so the extreme it left behind is a swing — and asking
        // about the wreckage first would throw that swing away.
        if let Some(step) = self.look(bar, now, rules, memory)? {
            return Ok(step);
        }

        // The run has been undone completely and proved nothing on the way —
        // too small to count, most likely. It simply restarts from here.
        if is_behind(self.hunting, backward, self.anchor.price) {
            self.anchor = Extreme::new(backward, now);
            self.extreme = Extreme::new(forward, now);
            self.retrace = Extreme::new(backward, now);

            return Ok(Step::Continue);
        }

        self.absorb(bar, now);

        Ok(Step::Continue)
    }

    /// Has anything proved itself on this candle? **Changes nothing.**
    ///
    /// Kept separate from `take` because the seed asks the same question of a
    /// leg it rebuilds from scratch each candle.
    pub(super) fn look(
        &self,
        bar: &Bar,
        now: DateTime<Utc>,
        rules: &Rules,
        memory: &RunMemory,
    ) -> Result<Option<Step>, SwingError> {
        let (forward, backward) = sides(self.hunting, bar);

        // Price has taken the old extreme out. If it had already given back
        // enough on the way, that pause was a real pullback and both ends of
        // it are swings.
        if is_beyond(self.hunting, forward, self.extreme.price) {
            return self.by_resumption(forward, backward, now, rules, memory);
        }

        // Otherwise: does this candle deepen the give-back far enough to prove
        // the extreme on its own?
        let mut deepened = self.clone();
        deepened.absorb(bar, now);

        deepened.by_depth(forward, now, rules, memory)
    }

    /// Takes the candle in without deciding anything.
    fn absorb(&mut self, bar: &Bar, now: DateTime<Utc>) {
        let (forward, backward) = sides(self.hunting, bar);

        if is_beyond(self.hunting, forward, self.extreme.price) {
            self.extreme = Extreme::new(forward, now);
            self.retrace = Extreme::new(backward, now);
        } else if is_behind(self.hunting, backward, self.retrace.price) {
            self.retrace = Extreme::new(backward, now);
        }
    }

    /// Enough given back for the extreme to prove itself on its own.
    fn by_depth(
        &self,
        forward: Decimal,
        now: DateTime<Utc>,
        rules: &Rules,
        memory: &RunMemory,
    ) -> Result<Option<Step>, SwingError> {
        // **A candle cannot confirm a peak it made itself.** The peak is only
        // knowable once a later candle has come back off it.
        if now <= self.extreme.bar_time
            || !self.spans_more_than_one_candle()
            || !self.given_back_enough(rules.confirm_retracement)
        {
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
    fn by_resumption(
        &self,
        forward: Decimal,
        backward: Decimal,
        now: DateTime<Utc>,
        rules: &Rules,
        memory: &RunMemory,
    ) -> Result<Option<Step>, SwingError> {
        let a_real_pause =
            self.retrace.bar_time > self.extreme.bar_time && now > self.retrace.bar_time;

        if !a_real_pause
            || !self.spans_more_than_one_candle()
            || !self.given_back_enough(rules.shallow_retracement)
        {
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

    fn run(&self) -> Decimal {
        span(self.anchor.price, self.extreme.price)
    }

    /// Has this run happened at all?
    ///
    /// **A run that starts and ends on the same candle is not a run** — it is
    /// one candle's height. Without this, two flat candles in a row would
    /// confirm a swing: the whole of that "run" gets given back inside the
    /// next candle, which passes every share test there is.
    ///
    /// It bites hardest at the very start of a history, where there is no
    /// memory of earlier runs to measure against.
    fn spans_more_than_one_candle(&self) -> bool {
        self.extreme.bar_time > self.anchor.bar_time
    }

    fn given_back_enough(&self, needed: Decimal) -> bool {
        let given_back = span(self.extreme.price, self.retrace.price);

        share(given_back, self.run()).is_some_and(|got| got >= needed)
    }
}
