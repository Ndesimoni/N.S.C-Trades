//! Levels from every timeframe at once, and which of them get a line.

use nsc_core::candle::Candle;
use nsc_core::level::Level;
use nsc_core::timeframe::{DayBoundary, Timeframe};
use rust_decimal::Decimal;

use super::finder::find_levels;
use crate::aggregate::aggregate;
use crate::config::{LevelSettings, SwingSettings};
use crate::error::TaError;
use crate::swings::find_swings;

/// Finds levels at several timeframes from one file of candles, and works out
/// which of them get drawn.
///
/// `base` is the file as it arrived. Each timeframe asked for is built up from
/// it through the same aggregator the live bot uses, so what comes out is what
/// the bot would have seen.
///
/// Timeframes smaller than the file are skipped. You cannot cut a daily candle
/// into 4-hour ones.
///
/// Everything found comes back. Ask [`Level::is_drawn`] before putting one on
/// a chart.
pub fn find_levels_across(
    base: &[Candle],
    base_timeframe: Timeframe,
    timeframes: &[Timeframe],
    boundary: &DayBoundary,
    swings: SwingSettings,
    levels: &LevelSettings,
    atr_period: usize,
) -> Result<Vec<Level>, TaError> {
    levels.validate()?;

    let mut found = Vec::new();

    for timeframe in timeframes {
        if *timeframe < base_timeframe {
            continue;
        }

        let candles = if *timeframe == base_timeframe {
            base.to_vec()
        } else {
            aggregate(base, base_timeframe, *timeframe, boundary)?
        };

        if candles.is_empty() {
            continue;
        }

        let swings_here = find_swings(&candles, swings.clone())?;
        found.extend(find_levels(
            &candles,
            &swings_here,
            *timeframe,
            levels,
            atr_period,
        )?);
    }

    decide_what_gets_a_line(found, levels)
}

/// Works out which levels get a line, and why the rest do not.
///
/// Two rules, in this order.
///
/// ## One: a timeframe does not crowd itself
///
/// Price chops around one area for two years and turns a dozen times, so the
/// finder sees a level at every turn. You look at all of it and draw **one**
/// line saying "price did something here".
///
/// So levels on the same timeframe must sit `min_separation_bands` apart. When
/// two are too close the one with **more touches** keeps the line — the price
/// where it actually turned, not the middle of the area.
///
/// ## Two: the bigger timeframe wins
///
/// A daily landing on a weekly is not drawn. You look at the weekly line and
/// you already know the price matters.
///
/// ## Why the order matters
///
/// Crowding is settled first, per timeframe, before anything is covered. A
/// weekly that got crowded out is not drawn, and **only a drawn level can
/// cover another** — so a daily sitting at that price is left uncovered and
/// draws itself in blue.
///
/// That is the demotion: a second weekly that could not have its own line
/// becomes a daily line instead, without needing a rule of its own.
///
/// ## Nothing is deleted
///
/// Every level comes back. Two timeframes turning at one price is confluence,
/// and confluence is the reason the price is worth trading.
pub fn decide_what_gets_a_line(
    found: Vec<Level>,
    settings: &LevelSettings,
) -> Result<Vec<Level>, TaError> {
    // Biggest timeframe first so the bigger ones are settled before the
    // smaller ones are judged against them. Within a timeframe, most touches
    // first — that is the one that keeps the line.
    let mut queue = found;
    queue.sort_by_key(|level| {
        (
            std::cmp::Reverse(level.timeframe()),
            std::cmp::Reverse(level.touches().unwrap_or(0)),
        )
    });

    let mut settled: Vec<Level> = Vec::with_capacity(queue.len());

    for level in queue {
        // Rule one: does this timeframe already have a line near here?
        let crowded = settled.iter().any(|kept| {
            kept.is_drawn()
                && kept.timeframe() == level.timeframe()
                && gap(*kept, level) <= room(*kept, settings.min_separation_bands)
        });

        if crowded {
            settled.push(level.crowded_out());
            continue;
        }

        // Rule two: is a bigger timeframe already covering this price?
        let cover = settled
            .iter()
            .filter(|bigger| bigger.is_drawn() && bigger.timeframe() > level.timeframe())
            .find(|bigger| gap(**bigger, level) <= room(**bigger, settings.absorb_gap_bands));

        match cover {
            Some(bigger) => settled.push(level.covered_by(bigger.timeframe())?),
            None => settled.push(level),
        }
    }

    settled.sort_by_key(|level| level.band().low().value());

    Ok(settled)
}

/// How far apart two bands are. Zero or less means they overlap.
fn gap(a: Level, b: Level) -> Decimal {
    let above = (b.band().low() - a.band().high()).value();
    let below = (a.band().low() - b.band().high()).value();

    above.max(below)
}

/// How much clearance a level demands, in its own band-widths.
///
/// Measured against the level already on the chart, so a weekly demands more
/// room than a 4-hour does. That is why the settings are in bands rather than
/// points: they mean the same thing on gold and on EURUSD.
fn room(around: Level, bands: f64) -> Decimal {
    match Decimal::from_f64_retain(bands) {
        Some(bands) => around.band().thickness().value() * bands,
        None => Decimal::ZERO,
    }
}
