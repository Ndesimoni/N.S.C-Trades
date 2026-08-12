//! Building bigger candles one smaller candle at a time.

use chrono::{DateTime, Utc};
use nsc_core::candle::Candle;
use nsc_core::error::CoreError;
use nsc_core::timeframe::{DayBoundary, Timeframe};

use super::bucket::Bucket;
use crate::error::TaError;

/// Turns a stream of smaller candles into bigger ones.
///
/// Feed it complete candles in time order. It gives back a bigger candle only
/// when one has genuinely finished — which is when a candle belonging to the
/// next bucket arrives.
///
/// ## Why arrival rather than the clock
///
/// A 4-hour candle is not finished because four hours have passed. It is
/// finished because the next one has started. The market can be shut, a feed
/// can be late, a session can end early on Christmas Eve — and a candle that
/// is merely expected is not a candle that happened.
///
/// The consequence is that the newest bigger candle is always still forming.
/// Ask for it with [`forming`](Self::forming), which hands it back marked
/// incomplete so nothing downstream can mistake it for history.
#[derive(Debug, Clone)]
pub struct Aggregator {
    into: Timeframe,
    boundary: DayBoundary,
    open: Option<Bucket>,
    last_seen: Option<DateTime<Utc>>,
}

impl Aggregator {
    /// Refuses to build candles that are not bigger than the ones going in.
    ///
    /// Building 15-minute candles out of 4-hour ones is not a hard job, it is
    /// a meaningless one — and it would quietly produce a chart that looks
    /// perfectly fine.
    pub fn new(from: Timeframe, into: Timeframe, boundary: DayBoundary) -> Result<Self, TaError> {
        if into <= from {
            return Err(TaError::CannotAggregate {
                from: from.to_string(),
                into: into.to_string(),
            });
        }

        Ok(Self {
            into,
            boundary,
            open: None,
            last_seen: None,
        })
    }

    /// Takes the next smaller candle.
    ///
    /// Gives back a bigger candle on the candle that ends one — meaning the
    /// bigger candle just handed back is finished, and a new one has started
    /// with the candle you passed in.
    pub fn update(&mut self, candle: &Candle) -> Result<Option<Candle>, TaError> {
        if !candle.is_complete() {
            return Err(TaError::IncompleteCandle {
                open_time: candle.open_time(),
            });
        }

        // Out of order, the bucket about to be sealed would be sealed by a
        // candle from BEFORE it, and the history handed back would run
        // backwards. Nothing downstream checks that, so it is checked here.
        if let Some(last) = self.last_seen
            && candle.open_time() <= last
        {
            return Err(TaError::Core(CoreError::CandlesOutOfOrder {
                arriving: candle.open_time(),
                last,
            }));
        }
        self.last_seen = Some(candle.open_time());

        let belongs_to = self.into.candle_start(candle.open_time(), &self.boundary)?;

        match &mut self.open {
            // Still inside the same bigger candle.
            Some(bucket) if bucket.starts_at() == belongs_to => {
                bucket.take(candle);
                Ok(None)
            }

            // A new bucket has started, so the one before it is finished —
            // and only now is that knowable.
            Some(bucket) => {
                let finished = bucket.seal(true)?;
                self.open = Some(Bucket::open(belongs_to, candle));
                Ok(Some(finished))
            }

            None => {
                self.open = Some(Bucket::open(belongs_to, candle));
                Ok(None)
            }
        }
    }

    /// The bigger candle currently being built, marked **incomplete**.
    ///
    /// For drawing a live chart, and for nothing else. Its high and low have
    /// not finished happening, and every analysis in this project refuses an
    /// incomplete candle on purpose.
    pub fn forming(&self) -> Result<Option<Candle>, TaError> {
        self.open
            .as_ref()
            .map(|bucket| bucket.seal(false))
            .transpose()
    }
}
