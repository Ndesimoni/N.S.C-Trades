//! Reading `config/`.

use std::path::Path;

use anyhow::{Context, Result};
use rust_decimal::Decimal;
use serde::Deserialize;

use super::{Band, Timeframe};

/// How thick a band is, per timeframe, as a share of a normal candle.
///
/// The same on every pair — from `config/levels.toml`.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Thickness {
    pub weekly: Decimal,
    pub daily: Decimal,
    pub h4: Decimal,
}

impl Thickness {
    pub fn of(&self, timeframe: Timeframe) -> Decimal {
        match timeframe {
            Timeframe::Weekly => self.weekly,
            Timeframe::Daily => self.daily,
            Timeframe::H4 => self.h4,
        }
    }
}

/// One pair, as its file describes it.
///
/// **The file being there is why the pair is watched.** Add a file, the pair
/// gets pulled; delete it, it stops.
#[derive(Debug, Clone, Deserialize)]
pub struct Pair {
    pub symbol: String,
    pub digits: u32,
    #[serde(default)]
    pub nightly_break_minutes: i64,
    #[serde(default, rename = "level")]
    pub levels: Vec<Line>,
}

/// One line he drew. A price and which chart it was on — nothing else, because
/// nothing else is his to decide.
#[derive(Debug, Clone, Deserialize)]
pub struct Line {
    pub timeframe: Timeframe,
    /// Text on purpose, so a price survives the trip through a file exactly.
    /// A number in TOML goes through a float and 4094.10 stops being 4094.10.
    pub price: Decimal,
}

impl Pair {
    /// Every line, turned into a band.
    ///
    /// `candles` says how big a normal candle is on each timeframe. A level
    /// whose timeframe is missing from it is skipped rather than guessed at —
    /// a band of the wrong thickness is worse than no band, because it still
    /// looks like a level.
    pub fn bands(&self, thickness: Thickness, candles: &[(Timeframe, Decimal)]) -> Vec<Band> {
        self.levels
            .iter()
            .filter_map(|line| {
                let candle = candles
                    .iter()
                    .find(|(timeframe, _)| *timeframe == line.timeframe)
                    .map(|(_, size)| *size)?;

                Some(Band::around(
                    line.timeframe,
                    line.price,
                    candle,
                    thickness.of(line.timeframe),
                ))
            })
            .collect()
    }
}

pub fn load_thickness(path: &Path) -> Result<Thickness> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("could not read {}", path.display()))?;

    toml::from_str(&text)
        .with_context(|| format!("{} is not readable as thicknesses", path.display()))
}

pub fn load_pair(path: &Path) -> Result<Pair> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("could not read {}", path.display()))?;

    toml::from_str(&text).with_context(|| format!("{} is not readable as a pair", path.display()))
}
