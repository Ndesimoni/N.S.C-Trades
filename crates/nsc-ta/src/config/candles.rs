//! From the `[candles]` section of `config/ta.toml`.

use serde::{Deserialize, Serialize};

use crate::error::TaError;

/// What each candlestick shape has to measure to count as one.
///
/// **These are textbook numbers, not the trader's own.** They were taken as
/// standard defaults on 12 Aug 2026 to get the detectors working, and they
/// stay marked as borrowed until a pair of charts — one taken, one passed —
/// replaces them. See `docs/worksheets/candles.md`.
///
/// Shape is measured as shares of the candle's own height, so no ATR and no
/// pip size come into it. The one exception is `belt_hold_min_atr_multiple`:
/// whether a candle is *big* is a different question from what shape it is,
/// and size is measured in normal candles like everything else here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandleSettings {
    /// How many times the body the long wick must be, for a pin bar.
    pub pin_min_tail_to_body: f64,

    /// How far into a pin bar the body may reach, measured from the end away
    /// from the tail — the nose and the body added together.
    ///
    /// This is what puts the body AT the far end rather than merely making it
    /// small. A candle with a quarter nose, a quarter body and a half tail has
    /// a small body sitting in the middle, and that is a spinning top leaning
    /// one way, not a rejection.
    pub pin_max_body_share: f64,

    /// The most the wick on the other side — the nose — may take up on its
    /// own.
    pub pin_max_nose_share: f64,

    /// The least the first candle's body may be, for an engulfing pair.
    ///
    /// Without it almost anything engulfs a doji, and "engulfing" stops
    /// meaning anything.
    pub engulfing_min_first_body_share: f64,

    /// The most of its candle a body may take and still count as a doji.
    pub doji_max_body_share: f64,

    /// A wick smaller than this counts as no wick at all. It is what separates
    /// a dragonfly from a long-legged doji.
    pub doji_max_missing_wick_share: f64,

    /// The most wick allowed on the opening side of a belt-hold. In real forex
    /// there is nearly always a tick or two, so "none" cannot mean exactly
    /// zero.
    pub belt_hold_max_open_wick_share: f64,

    /// The least of its candle a belt-hold's body must take up.
    pub belt_hold_min_body_share: f64,

    /// How tall a belt-hold must be, in normal candles. "A long candle" is
    /// about size, so this one is measured in ATR.
    pub belt_hold_min_atr_multiple: f64,

    /// How close two highs must be to count as the same price, in normal
    /// candles. Tweezers are never identical to the tick.
    pub tweezer_tolerance_atr: f64,
}

impl CandleSettings {
    pub fn validate(&self) -> Result<(), TaError> {
        let shares = [
            ("pin_max_body_share", self.pin_max_body_share),
            ("pin_max_nose_share", self.pin_max_nose_share),
            (
                "engulfing_min_first_body_share",
                self.engulfing_min_first_body_share,
            ),
            ("doji_max_body_share", self.doji_max_body_share),
            (
                "doji_max_missing_wick_share",
                self.doji_max_missing_wick_share,
            ),
            (
                "belt_hold_max_open_wick_share",
                self.belt_hold_max_open_wick_share,
            ),
            ("belt_hold_min_body_share", self.belt_hold_min_body_share),
        ];

        for (name, value) in shares {
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                return Err(TaError::BadSetting {
                    setting: format!("candles.{name}"),
                    value: value.to_string(),
                    why: "must be a share of a candle, between 0 and 1".into(),
                });
            }
        }

        if self.pin_min_tail_to_body <= 1.0 || !self.pin_min_tail_to_body.is_finite() {
            return Err(TaError::BadSetting {
                setting: "candles.pin_min_tail_to_body".into(),
                value: self.pin_min_tail_to_body.to_string(),
                why: "a wick no longer than its body is not a pin bar".into(),
            });
        }

        if self.doji_max_body_share >= self.pin_max_body_share {
            return Err(TaError::BadSetting {
                setting: "candles.doji_max_body_share".into(),
                value: self.doji_max_body_share.to_string(),
                why: "a doji has less body than a pin bar, or every doji is also a pin bar".into(),
            });
        }

        Ok(())
    }
}
