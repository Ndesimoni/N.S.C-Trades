use std::path::Path;

use super::LevelError;
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

    /// How close to a band counts as arriving at it, **in pips**.
    ///
    /// Small on purpose. The band's own edge is already hours of movement away
    /// from the line he drew, so this is slack for rounding rather than time to
    /// react — see the note in `config/levels.toml`.
    #[serde(default = "one_pip")]
    pub approach_pips: Decimal,

    /// How far into a band stops being a graze and becomes a real push, as a
    /// share of the band's own thickness.
    ///
    /// A wick that touched the edge and a candle that drove halfway in are not
    /// the same event, and the card says which.
    #[serde(default = "a_quarter")]
    pub kiss_depth: Decimal,
}

/// What `kiss_depth` is when a file predates it.
fn a_quarter() -> Decimal {
    Decimal::new(25, 2)
}

/// What `approach_pips` is when a file predates it.
fn one_pip() -> Decimal {
    Decimal::ONE
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

    /// How close counts as approaching **this** pair's bands, in pips.
    ///
    /// Missing means use the one in `config/levels.toml`. It is here so gold
    /// can be given more room than the euro without touching every other pair
    /// — four pips is two minutes of gold and nearly an hour of euro.
    #[serde(default)]
    pub approach_pips: Option<Decimal>,

    #[serde(default, rename = "level")]
    pub levels: Vec<Line>,
}

/// One line he drew. A price and which chart it was on — nothing else, because
/// nothing else is his to decide.
///
/// **Comparable**, so the watcher can tell whether a file it has already read
/// actually changed. Re-sizing a band costs a request, and re-building a
/// `Watch` forgets which zones price was already sitting in.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Line {
    pub timeframe: Timeframe,
    /// Text on purpose, so a price survives the trip through a file exactly.
    /// A number in TOML goes through a float and 4094.10 stops being 4094.10.
    pub price: Decimal,
}

impl Pair {
    /// One pip for this pair, as a price.
    ///
    /// **A pip is ten ticks**, so it falls straight out of `digits` and never
    /// needs a setting of its own. Gold is quoted to 2 decimals, so a pip is
    /// 0.10. The euro to 5, so a pip is 0.0001. The yen to 3, so 0.01.
    pub fn pip(&self) -> Decimal {
        Decimal::new(1, self.digits.saturating_sub(1))
    }

    /// How near price has to get before it counts as having arrived at a band.
    ///
    /// This pair's own `approach_pips` wins if it has one, otherwise the
    /// shared setting.
    pub fn reach(&self, thickness: Thickness) -> Decimal {
        let pips = self.approach_pips.unwrap_or(thickness.approach_pips);

        self.pip() * pips
    }

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

pub fn load_thickness(path: &Path) -> Result<Thickness, LevelError> {
    read_toml(path, "thicknesses")
}

pub fn load_pair(path: &Path) -> Result<Pair, LevelError> {
    read_toml(path, "a pair")
}

/// Reads a file and turns it into whatever was asked for.
///
/// **Gives up rather than guessing.** A levels file that half-parses would put
/// bands at prices he never drew, and every signal after that inherits it.
fn read_toml<T: serde::de::DeserializeOwned>(
    path: &Path,
    expected: &'static str,
) -> Result<T, LevelError> {
    let text = std::fs::read_to_string(path).map_err(|trouble| LevelError::CannotRead {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })?;

    toml::from_str(&text).map_err(|trouble| LevelError::NotReadable {
        path: path.display().to_string(),
        expected,
        detail: trouble.to_string(),
    })
}
