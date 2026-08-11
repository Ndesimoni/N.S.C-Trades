//! Turning a run of swing points into levels.

use chrono::{DateTime, Utc};
use nsc_core::candle::Candle;
use nsc_core::level::{Band, Level};
use nsc_core::price::{AtrMultiple, PriceDistance};
use nsc_core::swing::Swing;
use nsc_core::timeframe::Timeframe;
use rust_decimal::Decimal;

use super::grouping::{Group, best_group};
use crate::config::LevelSettings;
use crate::error::TaError;
use crate::indicators::atr::atr_series;

/// Finds every level on this timeframe.
///
/// Feed it the candles and the swings found in them. It gives back the bands
/// that had enough touches, ordered from the lowest price up.
///
/// ## What it ignores
///
/// Swings older than `max_age_bars`, and swings that had not confirmed by the
/// last candle. The second is the lookahead guard: a swing you could not have
/// seen yet cannot become part of a level you trade today.
///
/// ## The band thickness
///
/// Worked out once, from ATR at the last candle — how big a normal candle is
/// *now*. So a level drawn on gold and a level drawn on EURUSD get bands that
/// are the right size for each, from one setting.
pub fn find_levels(
    candles: &[Candle],
    swings: &[Swing],
    timeframe: Timeframe,
    settings: &LevelSettings,
    atr_period: usize,
) -> Result<Vec<Level>, TaError> {
    settings.validate()?;

    let Some(newest) = candles.last() else {
        return Ok(Vec::new());
    };

    if !newest.is_complete() {
        return Err(TaError::IncompleteCandle {
            open_time: newest.open_time(),
        });
    }

    let thickness = band_thickness(candles, settings, atr_period)?;
    let mut pool = recent_and_known(candles, swings, settings, newest.open_time());
    let mut found = Vec::new();

    while let Some(group) = best_group(&pool, thickness) {
        if group.swings.len() < settings.min_touches {
            break;
        }

        found.push(level_from(&group, thickness, timeframe)?);
        pool.retain(|swing| !group.swings.contains(swing));
    }

    found.sort_by_key(|level| level.centre());

    Ok(found)
}

/// How thick every band on this timeframe is, in real prices.
fn band_thickness(
    candles: &[Candle],
    settings: &LevelSettings,
    atr_period: usize,
) -> Result<PriceDistance, TaError> {
    let atr_now = atr_series(candles, atr_period)?
        .into_iter()
        .flatten()
        .next_back();

    // No ATR means the history is shorter than the ATR period, so there is no
    // idea yet what a normal candle looks like on this instrument. A band
    // needs that to be sized at all.
    let Some(atr_now) = atr_now else {
        return Err(TaError::NotEnoughCandles {
            needed: atr_period + 1,
            have: candles.len(),
        });
    };

    Ok(AtrMultiple::new(settings.band_atr_multiple).to_distance(atr_now)?)
}

/// The swings that are allowed to form levels: recent enough, and already
/// confirmed by the time of the last candle.
fn recent_and_known(
    candles: &[Candle],
    swings: &[Swing],
    settings: &LevelSettings,
    now: DateTime<Utc>,
) -> Vec<Swing> {
    // Counted in candles rather than in days, so a weekend or a market
    // holiday does not quietly shorten how far back the bot looks.
    let first_kept = candles.len().saturating_sub(settings.max_age_bars);

    let Some(oldest) = candles.get(first_kept) else {
        return Vec::new();
    };

    swings
        .iter()
        .filter(|swing| swing.bar_time() >= oldest.open_time() && swing.is_known_at(now))
        .copied()
        .collect()
}

/// Builds the level a group of swing points describes.
///
/// The band is centred between the lowest and highest swing in the group and
/// given the standard thickness. It is not stretched to fit them — it does not
/// need to be, because the group was chosen to fit inside one band already.
fn level_from(
    group: &Group,
    thickness: PriceDistance,
    timeframe: Timeframe,
) -> Result<Level, TaError> {
    let half_spread = PriceDistance::new(group.spread.value() / Decimal::from(2));
    let centre = group.lowest.price() + half_spread;

    let first_touch = group
        .swings
        .iter()
        .map(|s| s.bar_time())
        .min()
        .unwrap_or(group.lowest.bar_time());

    let last_touch = group
        .swings
        .iter()
        .map(|s| s.bar_time())
        .max()
        .unwrap_or(group.lowest.bar_time());

    // The level is knowable only once its last touch has confirmed as a
    // swing. That is what stops a level being traded before it existed.
    let confirmed_at = group
        .swings
        .iter()
        .map(|s| s.confirmed_at())
        .max()
        .unwrap_or(group.lowest.confirmed_at());

    let band = Band::around(centre, thickness)?;
    let touches = group.swings.len().try_into().unwrap_or(u32::MAX);

    Ok(Level::new(
        band,
        timeframe,
        touches,
        first_touch,
        last_touch,
        confirmed_at,
    )?)
}
