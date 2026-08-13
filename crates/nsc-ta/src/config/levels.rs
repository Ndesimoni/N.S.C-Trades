//! From the `[levels]` section of `config/ta.toml`.

use serde::{Deserialize, Serialize};

use crate::error::TaError;

/// How support and resistance get drawn.
///
/// All three of these decide what the bot *sees*. None of them decides what
/// to do about it — how many touches makes a level worth trading lives in
/// `config/strategy.toml`, because that is a trading opinion and this is the
/// chart-reading code.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelSettings {
    /// How thick a band is, as a fraction of a normal candle.
    ///
    /// Too thin and a wick that overshoots by a hair misses the level. Too
    /// thick and every price is at every level, so the check stops meaning
    /// anything.
    pub band_atr_multiple: f64,

    /// How many swing points must sit inside a band before it counts as a
    /// level at all.
    ///
    /// Two is the lowest that can mean anything — one swing point is just a
    /// swing point.
    pub min_touches: usize,

    /// How far back to look, in candles.
    ///
    /// Swings older than this are not considered. A price that turned the
    /// market three years ago and has not been near it since is history, not
    /// a level.
    pub max_age_bars: usize,

    /// How far clear of a bigger level a smaller one must sit to keep its own
    /// line, measured in the **bigger** band's own thickness.
    ///
    /// Overlapping is not the only thing that makes two lines read as one. A
    /// daily band sitting just under a weekly band, not touching it, still
    /// looks like one thick line and tells you nothing the weekly did not.
    ///
    /// So the bigger band is grown by this much on each side, and anything
    /// falling inside the grown zone loses its line.
    ///
    /// ```text
    /// 0.0   only when they actually touch
    /// 1.0   also within a full band's width
    /// 1.5   the setting: a daily must be one and a half weekly bands clear
    /// ```
    ///
    /// Measured in bands rather than price so it means the same thing on a
    /// weekly chart of gold and a 4-hour chart of EURUSD. A weekly band is
    /// thick, so "close to a weekly" is a bigger distance than "close to a
    /// 4-hour" — which is right.
    ///
    /// A drawing rule only. The level is kept either way — two timeframes
    /// turning at one price is confluence, and confluence is the reason the
    /// price is worth trading.
    pub absorb_gap_bands: f64,

    /// How far apart two levels on the SAME timeframe must sit, in that
    /// timeframe's own band-widths.
    ///
    /// The consolidation rule. Price chops around one area for two years and
    /// turns a dozen times, so a level is found at every turn. You look at all
    /// of it and draw one line saying "price did something here".
    ///
    /// When two are too close, the one with more touches keeps the line — the
    /// price where it actually turned, not the middle of the area.
    ///
    /// The one crowded out is not deleted. And because only a DRAWN level can
    /// cover a smaller one, a weekly that lost its line stops hiding the daily
    /// at that price — so the daily draws itself instead. That is the demotion,
    /// and it needs no rule of its own.
    pub min_separation_bands: f64,
}

impl LevelSettings {
    pub fn validate(&self) -> Result<(), TaError> {
        if self.band_atr_multiple <= 0.0 || !self.band_atr_multiple.is_finite() {
            return Err(TaError::BadSetting {
                setting: "levels.band_atr_multiple".into(),
                value: self.band_atr_multiple.to_string(),
                why: "a band with no thickness catches nothing".into(),
            });
        }

        if self.min_touches < 2 {
            return Err(TaError::BadSetting {
                setting: "levels.min_touches".into(),
                value: self.min_touches.to_string(),
                why: "one swing point on its own is not a level".into(),
            });
        }

        if self.max_age_bars == 0 {
            return Err(TaError::BadSetting {
                setting: "levels.max_age_bars".into(),
                value: "0".into(),
                why: "looking back no candles at all finds nothing".into(),
            });
        }

        if self.min_separation_bands < 0.0 || !self.min_separation_bands.is_finite() {
            return Err(TaError::BadSetting {
                setting: "levels.min_separation_bands".into(),
                value: self.min_separation_bands.to_string(),
                why: "a negative separation has no meaning".into(),
            });
        }

        if self.absorb_gap_bands < 0.0 || !self.absorb_gap_bands.is_finite() {
            return Err(TaError::BadSetting {
                setting: "levels.absorb_gap_bands".into(),
                value: self.absorb_gap_bands.to_string(),
                why: "a negative clearance has no meaning; zero means touching only".into(),
            });
        }

        Ok(())
    }
}
