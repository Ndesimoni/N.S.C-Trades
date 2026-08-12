//! One shape, found on one candle.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::shape::{Bias, CandleShape};
use crate::candle::Proportions;
use crate::error::CoreError;

/// A pattern the chart made, and how pronounced it was.
///
/// **The measurements travel with it on purpose.** A pin bar whose wick is
/// nine times its body and one that scrapes past the minimum are both pin
/// bars, and they are not the same candle. A rules layer that only hears "pin
/// bar" cannot tell them apart, and it cannot get the numbers back afterwards.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PatternSighting {
    shape: CandleShape,
    bias: Bias,

    /// The candle it completed on. For a two-candle shape, the second one.
    ///
    /// This is the first moment the pattern could have been acted on — the
    /// candle has closed, so the shape is final.
    at: DateTime<Utc>,

    /// How many candles it covers. One or two.
    spans: u8,

    /// The shape of the candle it completed on.
    proportions: Proportions,
}

impl PatternSighting {
    pub fn new(
        shape: CandleShape,
        bias: Bias,
        at: DateTime<Utc>,
        spans: u8,
        proportions: Proportions,
    ) -> Result<Self, CoreError> {
        if spans == 0 {
            return Err(CoreError::ImpossiblePattern {
                detail: "a pattern made of no candles is not a pattern".into(),
            });
        }

        Ok(Self {
            shape,
            bias,
            at,
            spans,
            proportions,
        })
    }

    pub fn shape(self) -> CandleShape {
        self.shape
    }

    pub fn bias(self) -> Bias {
        self.bias
    }

    pub fn at(self) -> DateTime<Utc> {
        self.at
    }

    pub fn spans(self) -> u8 {
        self.spans
    }

    /// The shape of the candle it completed on, as shares of that candle's
    /// own height.
    pub fn proportions(self) -> Proportions {
        self.proportions
    }
}
