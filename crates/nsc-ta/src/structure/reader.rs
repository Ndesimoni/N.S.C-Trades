//! Reading trend one candle at a time.

use std::cmp::{max, min};

use chrono::{DateTime, Utc};

use nsc_core::candle::Candle;
use nsc_core::price::Price;
use nsc_core::structure::{StructureEvent, Trend};
use nsc_core::swing::{Swing, SwingKind};

use super::watch::Marker;
use crate::config::StructureSettings;
use crate::error::TaError;

/// Works out which way the market is going, as candles arrive.
///
/// Hand it each closed candle along with any swings that confirmed on it. It
/// gives back a break of structure on the candle where one completes.
///
/// ## What counts as taken
///
/// Crossing the old extreme starts the test. Carrying a share of the previous
/// run past it finishes the test. Nothing in between counts, which is what
/// keeps the bot out of the poke that looks like a breakout and turns straight
/// back down.
///
/// If price crosses and stalls, the extreme stays on the books. A later candle
/// that carries far enough still completes the break — the test is about how
/// far price got, not about how quickly.
///
/// ## What this does not do
///
/// **Change of character** — the first swing that breaks the pattern, the
/// earliest hint that a trend is turning — is not here. It has not been
/// described yet, and inventing it would put a rule in the bot that nobody
/// agreed to.
#[derive(Debug, Clone)]
pub struct StructureReader {
    settings: StructureSettings,

    high: Option<Marker>,
    low: Option<Marker>,

    /// The swing before the one that just confirmed, so the run behind it can
    /// be measured.
    previous: Option<Swing>,

    /// The lowest and highest price seen so far.
    ///
    /// Only used for the very first swing, which has no earlier swing behind
    /// it to measure a run from. Without this the bot would ignore the first
    /// high of every history — including the one it starts trading on.
    seen_low: Option<Price>,
    seen_high: Option<Price>,

    trend: Trend,
}

impl StructureReader {
    pub fn new(settings: StructureSettings) -> Result<Self, TaError> {
        settings.validate()?;

        Ok(Self {
            settings,
            high: None,
            low: None,
            previous: None,
            seen_low: None,
            seen_high: None,
            trend: Trend::Unclear,
        })
    }

    /// Which way the market is going, on the evidence so far.
    ///
    /// The direction of the last extreme properly taken out. `Unclear` until
    /// one has been, which is the honest answer for a chart that has not
    /// broken anything yet.
    pub fn trend(&self) -> Trend {
        self.trend
    }

    /// Takes the next candle, along with the swings that confirmed on it.
    ///
    /// The swings go in first: they were knowable by the close of this candle,
    /// so the extremes being watched are the newest ones.
    ///
    /// Usually gives back nothing. Two events can land together when one side
    /// is taken while a push at the other side gives up — an outside candle
    /// does exactly that.
    pub fn update(
        &mut self,
        candle: &Candle,
        confirmed: &[Swing],
    ) -> Result<Vec<StructureEvent>, TaError> {
        if !candle.is_complete() {
            return Err(TaError::IncompleteCandle {
                open_time: candle.open_time(),
            });
        }

        let mut events = Vec::new();
        for swing in confirmed {
            events.extend(self.remember(*swing, candle.open_time())?);
        }

        events.extend(self.look(candle)?);

        // Recorded last, so that a swing confirming on this candle measures
        // its run against the candles BEFORE it — which are the only ones that
        // could have made it.
        self.seen_low = Some(min(self.seen_low.unwrap_or(candle.low()), candle.low()));
        self.seen_high = Some(max(self.seen_high.unwrap_or(candle.high()), candle.high()));

        Ok(events)
    }

    /// Files a newly confirmed swing as the extreme to watch.
    ///
    /// The newest one replaces the one before it. "The previous high" means
    /// the most recent high, even when it is lower than the one before.
    ///
    /// If a push at the old extreme was still in flight, it is closed out and
    /// reported as failed. Price never got where it needed to, and dropping
    /// the record because a new swing happened to form would lose exactly the
    /// evidence these are collected for.
    fn remember(
        &mut self,
        swing: Swing,
        now: DateTime<Utc>,
    ) -> Result<Option<StructureEvent>, TaError> {
        // Normally the run behind a high is the move up from the low before
        // it. The first swing of a history has no swing before it, so the
        // lowest price seen so far stands in — that is where the move came up
        // from, as far as anything here can know.
        let started_at = match (self.previous, swing.kind()) {
            (Some(previous), _) => Some(previous.price()),
            (None, SwingKind::High) => self.seen_low,
            (None, SwingKind::Low) => self.seen_high,
        };

        self.previous = Some(swing);

        let Some(started_at) = started_at else {
            return Ok(None);
        };

        let marker = Marker::new(swing, started_at);
        let replaced = match marker.kind() {
            SwingKind::High => self.high.replace(marker),
            SwingKind::Low => self.low.replace(marker),
        };

        let Some(mut replaced) = replaced else {
            return Ok(None);
        };

        Ok(replaced.gave_up(now)?.map(StructureEvent::Failed))
    }

    /// Asks both extremes what this candle did to them.
    fn look(&mut self, candle: &Candle) -> Result<Vec<StructureEvent>, TaError> {
        let needed = self.settings.min_follow_through;
        let now = candle.open_time();
        let mut events = Vec::new();

        if let Some(mut marker) = self.high {
            let seen = marker.saw(candle.high(), now, needed)?;
            self.high = Some(marker);
            self.settle(seen, SwingKind::High, &mut events);
        }

        if let Some(mut marker) = self.low {
            let seen = marker.saw(candle.low(), now, needed)?;
            self.low = Some(marker);
            self.settle(seen, SwingKind::Low, &mut events);
        }

        Ok(events)
    }

    /// Files what happened. Only a break changes the trend — a failed attempt
    /// is evidence, not a direction.
    fn settle(
        &mut self,
        seen: Option<StructureEvent>,
        side: SwingKind,
        events: &mut Vec<StructureEvent>,
    ) {
        let Some(event) = seen else {
            return;
        };

        if event.is_taken() {
            match side {
                SwingKind::High => self.high = None,
                SwingKind::Low => self.low = None,
            }
            self.trend = Trend::from_break(side);
        }

        events.push(event);
    }
}
