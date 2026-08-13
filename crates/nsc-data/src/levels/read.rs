//! Reading a levels file off disk and turning it into levels.

use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};
use nsc_core::level::{Band, Level};
use nsc_core::price::{Price, PriceDistance};
use nsc_core::timeframe::Timeframe;
use rust_decimal::Decimal;

use super::file::{DrawnLevel, LevelsFile};
use crate::error::DataError;

/// Reads `config/levels/<PAIR>.toml`.
///
/// These are the trader's own levels. The bot trades these; `nsc-ta::levels`
/// runs alongside only so the finder can be scored against them.
///
/// Every level comes back tagged [`Origin::DrawnByHand`], with no touch count.
/// Asking one for its touches gives `None` rather than a made-up number.
///
/// [`Origin::DrawnByHand`]: nsc_core::level::Origin::DrawnByHand
///
/// ## Nothing is thinned
///
/// The crowding and covering rules in `nsc-ta` do not run on these. He already
/// does that thinning while drawing — he does not put two weeklies close
/// together, or a daily on top of a weekly. Applying the rules again could
/// only hide a level he chose to draw.
pub fn read_levels(path: &Path, thickness: &dyn Thickness) -> Result<Vec<Level>, DataError> {
    let text = std::fs::read_to_string(path).map_err(|e| DataError::CannotRead {
        path: path.to_path_buf(),
        detail: e.to_string(),
    })?;

    let file: LevelsFile = toml::from_str(&text).map_err(|e| DataError::BadLevelsFile {
        path: path.to_path_buf(),
        detail: e.to_string(),
    })?;

    file.level
        .into_iter()
        .map(|drawn| level_from(drawn, path, thickness))
        .collect()
}

/// How thick a band is on a given timeframe.
///
/// The file holds one price per level. The thickness is the same on every
/// chart he draws, so it belongs in the settings rather than being written out
/// a hundred times — and it needs a normal candle to work out, which this
/// crate has no business knowing about.
pub trait Thickness {
    /// The full height of a band on this timeframe. `None` if there is not
    /// enough history on that timeframe to know what a normal candle is.
    fn for_timeframe(&self, timeframe: Timeframe) -> Option<PriceDistance>;
}

fn level_from(
    drawn: DrawnLevel,
    path: &Path,
    thickness: &dyn Thickness,
) -> Result<Level, DataError> {
    let timeframe =
        Timeframe::from_str(&drawn.timeframe).map_err(|_| DataError::BadLevelsFile {
            path: path.to_path_buf(),
            detail: format!("'{}' is not a timeframe", drawn.timeframe),
        })?;

    let centre = price(drawn.price, path)?;

    let height = thickness
        .for_timeframe(timeframe)
        .ok_or_else(|| DataError::BadLevelsFile {
            path: path.to_path_buf(),
            detail: format!(
                "there is not enough {timeframe} history to know how thick a band should be"
            ),
        })?;

    let band = Band::around(centre, height).map_err(|e| DataError::BadLevelsFile {
        path: path.to_path_buf(),
        detail: format!("cannot put a band around {}: {e}", drawn.price),
    })?;

    let from = day_drawn(&drawn, path)?;

    Ok(Level::drawn_by_hand(band, timeframe, from))
}

/// Midnight UTC on the day it was drawn.
///
/// The day is enough. Nobody records the minute they drew a line, and a level
/// becoming usable a few hours early on the day it was drawn cannot flatter a
/// backtest — the point is that it does not reach backwards into last year.
fn day_drawn(drawn: &DrawnLevel, path: &Path) -> Result<DateTime<Utc>, DataError> {
    let bad = || DataError::BadLevelsFile {
        path: path.to_path_buf(),
        detail: format!("'{}' is not a day this program can read", drawn.from),
    };

    let date = drawn.from.date.ok_or_else(bad)?;

    let day = NaiveDate::from_ymd_opt(date.year as i32, date.month as u32, date.day as u32)
        .ok_or_else(bad)?;

    Ok(Utc.from_utc_datetime(&day.and_time(NaiveTime::MIN)))
}

fn price(value: f64, path: &Path) -> Result<Price, DataError> {
    Decimal::from_f64_retain(value)
        .map(Price::new)
        .ok_or_else(|| DataError::BadLevelsFile {
            path: path.to_path_buf(),
            detail: format!("{value} is not a price this program can hold"),
        })
}

/// Reads every levels file in a folder, keyed by instrument.
///
/// The file name is the instrument: `XAUUSD.toml` holds the gold levels. One
/// file per pair, so adding an instrument is dropping a file in.
pub fn read_all_levels(
    folder: &Path,
    thickness: &dyn Thickness,
) -> Result<Vec<(String, Vec<Level>)>, DataError> {
    let entries = std::fs::read_dir(folder).map_err(|e| DataError::CannotRead {
        path: folder.to_path_buf(),
        detail: e.to_string(),
    })?;

    let mut out = Vec::new();

    for entry in entries.flatten() {
        let path: PathBuf = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        let Some(symbol) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        out.push((symbol.to_string(), read_levels(&path, thickness)?));
    }

    out.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(out)
}
