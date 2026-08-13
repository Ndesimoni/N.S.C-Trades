//! The same findings, as JSON, for drawing elsewhere.
//!
//! Text is for reading on a terminal. This is for anything that wants to put
//! the swings and levels on a picture — which is how you actually check
//! whether they match the ones you would draw.
//!
//! ## Levels come from every timeframe, not just the one on screen
//!
//! There is one set of levels, not a set per chart. A weekly level is still a
//! weekly level when you are looking at the daily, and that is exactly why it
//! matters there.
//!
//! So each level carries the timeframe it was found on, and whether a bigger
//! one is already covering that price. Draw the ones where `drawn` is true.

use anyhow::Result;
use nsc_core::candle::Candle;
use nsc_core::price::{AtrMultiple, PriceDistance};
use nsc_core::timeframe::Timeframe;
use nsc_ta::indicators::atr::atr_series;

use super::settings::{candle_settings, level_settings, structure_settings, swing_settings};
use crate::{Args, boundary};

/// How thick a hand-drawn level's band is, per timeframe.
///
/// Worked out from a normal candle on each timeframe, built up from the file
/// through the same aggregator the bot uses. The fractions come from measuring
/// his own gold chart, and they are the same on every instrument — he draws
/// with the same pen everywhere.
struct DrawnThickness {
    weekly: Option<PriceDistance>,
    daily: Option<PriceDistance>,
    four_hour: Option<PriceDistance>,
}

impl nsc_data::levels::Thickness for DrawnThickness {
    fn for_timeframe(&self, timeframe: Timeframe) -> Option<PriceDistance> {
        match timeframe {
            Timeframe::W1 => self.weekly,
            Timeframe::D1 => self.daily,
            Timeframe::H4 => self.four_hour,
            _ => None,
        }
    }
}

fn drawn_thickness(candles: &[Candle], args: &Args) -> Result<DrawnThickness> {
    let base = args.from.parse::<Timeframe>().unwrap_or(Timeframe::M15);

    let of = |timeframe: Timeframe, fraction: f64| -> Option<PriceDistance> {
        let built = if timeframe == base {
            candles.to_vec()
        } else {
            nsc_ta::aggregate::aggregate(candles, base, timeframe, &boundary().ok()?).ok()?
        };

        let atr = atr_series(&built, args.atr_period)
            .ok()?
            .last()
            .copied()??;

        AtrMultiple::new(fraction).to_distance(atr).ok()
    };

    Ok(DrawnThickness {
        weekly: of(Timeframe::W1, 0.35),
        daily: of(Timeframe::D1, 0.60),
        four_hour: of(Timeframe::H4, 0.60),
    })
}

pub fn as_json(candles: &[Candle], timeframe: Timeframe, args: &Args) -> Result<()> {
    let swings = nsc_ta::swings::find_swings(candles, swing_settings(args))?;

    // Everything from the chart's own timeframe upwards. Smaller ones are
    // skipped inside — you cannot cut a daily candle into 4-hour pieces.
    let wanted = [Timeframe::H4, Timeframe::D1, Timeframe::W1];

    // His own levels if a file was given, otherwise the finder's guesses.
    let levels = match &args.levels {
        Some(path) => nsc_data::levels::read_levels(path, &drawn_thickness(candles, args)?)?,
        None => nsc_ta::levels::find_levels_across(
            candles,
            timeframe,
            &wanted,
            &boundary()?,
            swing_settings(args),
            &level_settings(args),
            args.atr_period,
        )?,
    };

    let events = nsc_ta::structure::read_structure(candles, &swings, &structure_settings())?;
    let patterns = nsc_ta::candles::find_patterns(candles, &candle_settings(), args.atr_period)?;

    let out = serde_json::json!({
        "timeframe": timeframe.to_string(),
        "candles": candles.iter().map(|candle| serde_json::json!({
            "t": candle.open_time().to_rfc3339(),
            "o": candle.open().value().to_string(),
            "h": candle.high().value().to_string(),
            "l": candle.low().value().to_string(),
            "c": candle.close().value().to_string(),
        })).collect::<Vec<_>>(),
        "swings": swings.iter().map(|swing| serde_json::json!({
            "kind": format!("{:?}", swing.kind()),
            "price": swing.price().value().to_string(),
            "at": swing.bar_time().to_rfc3339(),
            "known": swing.confirmed_at().to_rfc3339(),
        })).collect::<Vec<_>>(),
        "levels": levels.iter().map(|level| serde_json::json!({
            "timeframe": level.timeframe().to_string(),
            "origin": format!("{:?}", level.origin()),
            "low": level.band().low().value().to_string(),
            "high": level.band().high().value().to_string(),
            "touches": level.touches(),
            "first_touch": level.first_touch().map(|t| t.to_rfc3339()),
            "last_touch": level.last_touch().map(|t| t.to_rfc3339()),
            "drawn": level.is_drawn(),
            "absorbed_by": level.absorbed_by().map(|tf| tf.to_string()),
            "why_hidden": level.not_drawn().map(|r| format!("{r:?}")),
        })).collect::<Vec<_>>(),
        "breaks": events.iter().filter(|event| event.is_taken()).count(),
        "failed": events.iter().filter(|event| !event.is_taken()).count(),
        "patterns": patterns.len(),
    });

    println!("{}", serde_json::to_string(&out)?);

    Ok(())
}
