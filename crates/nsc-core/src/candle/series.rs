//! A run of candles for one instrument, on one timeframe.

use serde::{Deserialize, Serialize};

use super::bar::Candle;
use crate::error::CoreError;
use crate::symbol::Symbol;
use crate::timeframe::Timeframe;

/// Candles for one instrument on one timeframe, oldest first.
///
/// The instrument and the timeframe are stored here, once, instead of on
/// every candle. Two reasons.
///
/// **Space.** A backtest holds millions of candles. Writing "EURUSD" on each
/// one is a lot of room for a fact that never changes.
///
/// **Safety.** Every candle in a list like this is the same instrument,
/// because there is no way to put a different one in. You cannot end up with
/// gold candles hiding inside a list of EURUSD ones.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandleSeries {
    symbol: Symbol,
    timeframe: Timeframe,
    candles: Vec<Candle>,
}

impl CandleSeries {
    pub fn new(symbol: Symbol, timeframe: Timeframe) -> Self {
        Self {
            symbol,
            timeframe,
            candles: Vec::new(),
        }
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn timeframe(&self) -> Timeframe {
        self.timeframe
    }

    pub fn len(&self) -> usize {
        self.candles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candles.is_empty()
    }

    /// The most recent candle.
    pub fn last(&self) -> Option<&Candle> {
        self.candles.last()
    }

    /// All the candles, oldest first.
    pub fn candles(&self) -> &[Candle] {
        &self.candles
    }

    /// Builds a list from candles you already have — a year pulled out of
    /// the database, say.
    ///
    /// Checks the whole lot once, then stops checking. Loading old history
    /// and running live are different jobs, so neither pays for the other's
    /// checks.
    pub fn from_candles(
        symbol: Symbol,
        timeframe: Timeframe,
        candles: Vec<Candle>,
    ) -> Result<Self, CoreError> {
        for pair in candles.windows(2) {
            let (earlier, later) = (pair[0], pair[1]);

            // Catches both out-of-order and duplicates in one check.
            if later.open_time() <= earlier.open_time() {
                return Err(CoreError::CandlesOutOfOrder {
                    arriving: later.open_time(),
                    last: earlier.open_time(),
                });
            }
        }

        Ok(Self {
            symbol,
            timeframe,
            candles,
        })
    }

    /// Adds one candle to the end. This is what the live bot calls.
    ///
    /// Three things can happen.
    ///
    /// **The candle is newer than the last one.** It goes on the end. A jump
    /// in time is fine — weekends and holidays are real. Spotting gaps is
    /// `nsc-data`'s job, because it is the part that knows which gaps were
    /// expected.
    ///
    /// **Same time as the last one, and that one is still forming.** It
    /// replaces it. This is normal: a live candle updates again and again
    /// while it builds.
    ///
    /// **Same time as the last one, and that one has closed.** Refused. A
    /// closed candle is history. If history can change, you can run the same
    /// backtest twice and get two answers, with no way to tell which was
    /// right.
    pub fn push(&mut self, candle: Candle) -> Result<(), CoreError> {
        let Some(last) = self.candles.last().copied() else {
            self.candles.push(candle);
            return Ok(());
        };

        if candle.open_time() > last.open_time() {
            self.candles.push(candle);
            return Ok(());
        }

        if candle.open_time() < last.open_time() {
            return Err(CoreError::CandlesOutOfOrder {
                arriving: candle.open_time(),
                last: last.open_time(),
            });
        }

        // Same open_time as the last one.
        if last.is_complete() {
            return Err(CoreError::HistoryRewrite {
                open_time: candle.open_time(),
            });
        }

        let index = self.candles.len() - 1;
        self.candles[index] = candle;
        Ok(())
    }
}
