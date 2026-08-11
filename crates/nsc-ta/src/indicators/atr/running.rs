//! ATR kept up to date as candles arrive.

use nsc_core::candle::Candle;
use nsc_core::price::{Price, PriceDistance};
use rust_decimal::Decimal;

use crate::error::TaError;

/// Works out ATR as candles arrive.
///
/// Feed it complete candles in time order. It gives you the current ATR once
/// it has seen enough of them, and `None` before that.
#[derive(Debug, Clone, PartialEq)]
pub struct Atr {
    period: usize,

    /// Needed to spot gaps. `None` on the very first candle.
    previous_close: Option<Price>,

    /// True ranges collected while there are still too few candles to
    /// average. Thrown away once the first ATR is worked out.
    warmup: Vec<PriceDistance>,

    /// The running value, once there is one.
    current: Option<PriceDistance>,
}

impl Atr {
    /// `period` is `indicators.atr_period` from `config/ta.toml`. Fourteen is
    /// the usual choice and what most charts default to.
    pub fn new(period: usize) -> Result<Self, TaError> {
        if period < 2 {
            return Err(TaError::BadSetting {
                setting: "indicators.atr_period".into(),
                value: period.to_string(),
                why: "averaging needs at least two candles".into(),
            });
        }

        Ok(Self {
            period,
            previous_close: None,
            warmup: Vec::with_capacity(period),
            current: None,
        })
    }

    /// The ATR right now, or `None` if it has not seen enough candles yet.
    pub fn value(&self) -> Option<PriceDistance> {
        self.current
    }

    /// How many more candles are needed before there is a value.
    pub fn candles_still_needed(&self) -> usize {
        match self.current {
            Some(_) => 0,
            None => self.period - self.warmup.len(),
        }
    }

    /// Takes the next candle and gives back the new ATR.
    ///
    /// Returns `None` while warming up. That is normal at the start of any
    /// history — skip those candles and carry on.
    ///
    /// Refuses an unfinished candle. Its high and low have not happened yet,
    /// so letting one in would make ATR change under you, and every threshold
    /// in the system would move with it.
    pub fn update(&mut self, candle: &Candle) -> Result<Option<PriceDistance>, TaError> {
        if !candle.is_complete() {
            return Err(TaError::IncompleteCandle {
                open_time: candle.open_time(),
            });
        }

        let true_range = self.true_range(candle);
        self.previous_close = Some(candle.close());

        match self.current {
            // Already running. Nudge the value towards this candle:
            //
            //     new = (old x (period - 1) + this candle) / period
            //
            // One candle can only move it by a fraction. A single violent
            // candle must not convince the system that every candle is now
            // violent.
            Some(previous) => {
                let period = Decimal::from(self.period as u64);
                let weight = Decimal::from((self.period - 1) as u64);

                let smoothed = (previous.value() * weight + true_range.value()) / period;
                self.current = Some(PriceDistance::new(smoothed));
            }

            // Still warming up. Collect true ranges until there are enough,
            // then start from their plain average.
            None => {
                self.warmup.push(true_range);

                if self.warmup.len() == self.period {
                    let total: Decimal = self.warmup.iter().map(|tr| tr.value()).sum();
                    let average = total / Decimal::from(self.period as u64);

                    self.current = Some(PriceDistance::new(average));
                    self.warmup = Vec::new();
                }
            }
        }

        Ok(self.current)
    }

    /// The largest of: the candle's own height, and the two gaps from the
    /// previous close.
    ///
    /// On the very first candle there is no previous close, so it is just the
    /// height. That first value is a little too small — one more reason not
    /// to trust the first few candles of any history.
    fn true_range(&self, candle: &Candle) -> PriceDistance {
        let height = candle.range();

        match self.previous_close {
            None => height,
            Some(previous_close) => {
                let gap_up = (candle.high() - previous_close).abs();
                let gap_down = (candle.low() - previous_close).abs();

                height.max(gap_up).max(gap_down)
            }
        }
    }
}
