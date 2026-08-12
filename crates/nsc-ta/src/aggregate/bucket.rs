//! The smaller candles gathered so far for one bigger candle.

use chrono::{DateTime, Utc};
use nsc_core::candle::Candle;
use nsc_core::price::Price;

use crate::error::TaError;

/// One bigger candle under construction.
///
/// It knows its own start, so a smaller candle arriving can be checked against
/// the bucket it claims to belong to rather than assumed into it.
#[derive(Debug, Clone)]
pub(super) struct Bucket {
    starts_at: DateTime<Utc>,

    open: Price,
    high: Price,
    low: Price,
    close: Price,
}

impl Bucket {
    /// Starts a bigger candle from the first smaller one in it.
    pub fn open(starts_at: DateTime<Utc>, first: &Candle) -> Self {
        Self {
            starts_at,
            open: first.open(),
            high: first.high(),
            low: first.low(),
            close: first.close(),
        }
    }

    pub fn starts_at(&self) -> DateTime<Utc> {
        self.starts_at
    }

    /// Folds another smaller candle in.
    ///
    /// The open is never touched — it belongs to the first candle of the
    /// bucket, whatever arrives afterwards. The close is always the newest.
    pub fn take(&mut self, candle: &Candle) {
        self.high = self.high.max(candle.high());
        self.low = self.low.min(candle.low());
        self.close = candle.close();
    }

    /// The bigger candle, said to be finished or not.
    ///
    /// **`complete` is a decision, not a formality.** Pass `true` only once a
    /// candle from a later bucket has arrived. See the module docs for why the
    /// clock is not good enough.
    pub fn seal(&self, complete: bool) -> Result<Candle, TaError> {
        Ok(Candle::new(
            self.starts_at,
            self.open,
            self.high,
            self.low,
            self.close,
            None,
            complete,
        )?)
    }
}
