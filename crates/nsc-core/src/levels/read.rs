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

    /// How close to a band counts as arriving at it, **as a share of that
    /// band's own thickness**.
    ///
    /// Small on purpose. The band's own edge is already hours of movement away
    /// from the line he drew, so this is slack for rounding rather than time to
    /// react — see the note in `config/levels.toml`.
    ///
    /// **It was written in pips until 31 August 2026, and that was the last
    /// distance in this project that was.** Four pips is 22% of an AUD/USD
    /// daily band and 0.03% of a gold weekly one — the same setting meaning
    /// two entirely different things, which is what "never a pip number" is
    /// about. It gave him a pile of repeated cards on the Aussie and no
    /// approach warning at all on gold.
    /// **Deliberately no alias for the old `approach_pips`.** Four pips read
    /// as a share would be four times the band, and it would parse happily —
    /// the loudest possible bug arriving in total silence. An old file falls
    /// back to the default instead.
    #[serde(default = "a_twentieth")]
    pub approach_share: Decimal,

    /// How far into a band stops being a graze and becomes a real push, as a
    /// share of the band's own thickness.
    ///
    /// A wick that touched the edge and a candle that drove halfway in are not
    /// the same event, and the card says which.
    #[serde(default = "a_quarter")]
    pub kiss_depth: Decimal,

    /// **Only send a close card when the candle finished OUTSIDE the band.**
    ///
    /// His call, 26 August 2026. A candle that settled inside the zone is the
    /// most common of the three outcomes and the one that says least — price
    /// is there, undecided, and he already knew that from the approach alert.
    ///
    /// **It does not lose the rejection.** A wick into the zone that closed
    /// back out finishes above or below the band, so it still sends.
    ///
    /// Here rather than in the code because it is a preference about what he
    /// wants to hear, and preferences belong in `config/`.
    #[serde(default = "yes")]
    pub only_breaks: bool,

    /// **Which timeframes send a close card at all.**
    ///
    /// His call, 31 August 2026: *"we don't want those notifications from the
    /// one hour. The only notification we want from the one hour should be a
    /// setup."*
    ///
    /// **Setups are not affected.** A candlestick pattern at a zone still
    /// sends on every watched timeframe, which is the whole reason the 1-hour
    /// is watched at all.
    #[serde(default)]
    pub close_cards: ClosesOn,
}

/// Which timeframes are worth a close card.
///
/// **A struct rather than a list**, because `Thickness` is `Copy` and gets
/// passed by value everywhere. Two fields, because two timeframes are watched
/// — `closes/fetch.rs` says which, and a third here would be a setting for
/// something that never happens.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ClosesOn {
    /// **Off, and that is his answer rather than a default nobody chose.** The
    /// 1-hour sends setups and nothing else.
    #[serde(default)]
    pub h1: bool,

    #[serde(default = "yes")]
    pub h4: bool,

    /// **The daily, added 31 August 2026 at his word.** A daily candle closing
    /// at a level is the biggest close card the bot sends.
    #[serde(default = "yes")]
    pub d1: bool,
}

impl Default for ClosesOn {
    fn default() -> Self {
        ClosesOn {
            h1: false,
            h4: true,
            d1: true,
        }
    }
}

/// The default for `only_breaks` — his setting as of 26 August 2026.
fn yes() -> bool {
    true
}

/// What `kiss_depth` is when a file predates it.
fn a_quarter() -> Decimal {
    Decimal::new(25, 2)
}

/// What `approach_share` is when a file predates it.
fn a_twentieth() -> Decimal {
    Decimal::new(5, 2)
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
    pub approach_share: Option<Decimal>,

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
    /// This pair's own `approach_share` wins if it has one, otherwise the
    /// shared setting.
    pub fn reach_share(&self, thickness: Thickness) -> Decimal {
        self.approach_share.unwrap_or(thickness.approach_share)
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

impl Thickness {
    /// Does this timeframe send a close card?
    ///
    /// **Setups are never asked this.** A shape at a zone sends whatever the
    /// timeframe, and that is the one thing the 1-hour is watched for.
    ///
    /// Anything not watched for closes answers `false` rather than guessing.
    pub fn says_closes_on(&self, stored: &str) -> bool {
        match stored {
            "1h" => self.close_cards.h1,
            "4h" => self.close_cards.h4,
            "1d" => self.close_cards.d1,
            _ => false,
        }
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
