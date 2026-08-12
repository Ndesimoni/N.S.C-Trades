//! The same findings, as JSON, for drawing elsewhere.
//!
//! Text is for reading on a terminal. This is for anything that wants to put
//! the swings and levels on a picture — which is how you actually check
//! whether they match the ones you would draw.

use anyhow::Result;
use nsc_core::candle::Candle;
use nsc_core::timeframe::Timeframe;

use super::settings::{candle_settings, level_settings, structure_settings, swing_settings};
use crate::Args;

pub fn as_json(candles: &[Candle], timeframe: Timeframe, args: &Args) -> Result<()> {
    let swings = nsc_ta::swings::find_swings(candles, swing_settings(args))?;
    let levels = nsc_ta::levels::find_levels(
        candles,
        &swings,
        timeframe,
        &level_settings(),
        args.atr_period,
    )?;
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
            "low": level.band().low().value().to_string(),
            "high": level.band().high().value().to_string(),
            "touches": level.touches(),
        })).collect::<Vec<_>>(),
        "breaks": events.iter().filter(|event| event.is_taken()).count(),
        "failed": events.iter().filter(|event| !event.is_taken()).count(),
        "patterns": patterns.len(),
    });

    println!("{}", serde_json::to_string(&out)?);

    Ok(())
}
