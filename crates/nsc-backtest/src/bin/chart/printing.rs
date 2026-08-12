//! Printing what was found, for reading on a terminal.

use anyhow::{Context, Result};
use nsc_core::candle::Candle;
use nsc_core::price::PriceDistance;
use nsc_core::structure::StructureEvent;
use nsc_core::timeframe::Timeframe;
use nsc_ta::indicators::atr::atr_series;

use super::settings::{
    candle_settings, fib_settings, level_settings, structure_settings, swing_settings,
};
use crate::Args;

pub fn show_the_file(candles: &[Candle], args: &Args) -> Result<()> {
    let first = candles.first().context("at least one candle")?;
    let last = candles.last().context("at least one candle")?;

    println!("\n{}", args.file.display());
    println!(
        "{} candles, {} to {}",
        candles.len(),
        first.open_time().date_naive(),
        last.open_time().date_naive()
    );

    if let Some(atr) = latest_atr(candles, args.atr_period)? {
        println!("a normal candle right now is {}", atr.value().round_dp(5));
    }

    Ok(())
}

pub fn show_swings_and_levels(candles: &[Candle], timeframe: Timeframe, args: &Args) -> Result<()> {
    let swings = nsc_ta::swings::find_swings(candles, swing_settings(args))?;

    println!("\nSWINGS  {} found", swings.len());
    for swing in swings.iter().rev().take(8) {
        println!(
            "  {:?}  {}  on {}, usable from {}",
            swing.kind(),
            swing.price(),
            swing.bar_time().date_naive(),
            swing.confirmed_at().date_naive()
        );
    }

    let levels = nsc_ta::levels::find_levels(
        candles,
        &swings,
        timeframe,
        &level_settings(),
        args.atr_period,
    )?;

    println!("\nLEVELS  {} found, lowest first", levels.len());
    for level in &levels {
        println!(
            "  {} to {}   {} touches, last {}",
            level.band().low().round_for_display(5),
            level.band().high().round_for_display(5),
            level.touches(),
            level.last_touch().date_naive()
        );
    }

    Ok(())
}

pub fn show_structure(candles: &[Candle], args: &Args) -> Result<()> {
    let swings = nsc_ta::swings::find_swings(candles, swing_settings(args))?;
    let events = nsc_ta::structure::read_structure(candles, &swings, &structure_settings())?;

    let taken = events.iter().filter(|event| event.is_taken()).count();

    println!(
        "\nSTRUCTURE  {taken} extremes taken, {} pushes that failed",
        events.len() - taken
    );

    for event in events.iter().rev().take(6) {
        let share = event
            .share_of_run()
            .map(|share| format!("{:.0}% of the run", share * 100.0))
            .unwrap_or_else(|| "unmeasured".into());

        match event {
            StructureEvent::Taken(broken) => println!(
                "  TAKEN   {:?} at {} on {} — carried {share}",
                broken.kind(),
                broken.broken(),
                broken.at().date_naive()
            ),
            StructureEvent::Failed(attempt) => println!(
                "  FAILED  {:?} at {} by {} — got {share}",
                attempt.kind(),
                attempt.attempted(),
                attempt.to().date_naive()
            ),
        }
    }

    Ok(())
}

pub fn show_patterns(candles: &[Candle], args: &Args) -> Result<()> {
    let seen = nsc_ta::candles::find_patterns(candles, &candle_settings(), args.atr_period)?;

    println!("\nCANDLESTICKS  {} found", seen.len());
    for sighting in seen.iter().rev().take(8) {
        println!(
            "  {:?}  {:?}  on {}",
            sighting.shape(),
            sighting.bias(),
            sighting.at().date_naive()
        );
    }

    Ok(())
}

pub fn show_fibonacci(candles: &[Candle], args: &Args) -> Result<()> {
    let swings = nsc_ta::swings::find_swings(candles, swing_settings(args))?;
    let last = candles.last().context("at least one candle")?;

    let Some(measured) = nsc_ta::fibonacci::last_move(&swings, last.open_time())? else {
        println!("\nFIBONACCI  no completed leg to measure yet");
        return Ok(());
    };

    let settings = fib_settings();
    let Some(reading) = nsc_ta::fibonacci::FibReading::take(measured, last.close(), &settings)?
    else {
        println!("\nFIBONACCI  the move has no size");
        return Ok(());
    };

    println!(
        "\nFIBONACCI  measured {} to {}",
        measured.from(),
        measured.to()
    );
    println!("  0.382  {}", reading.strong_trend().round_for_display(5));
    println!("  0.5    {}", reading.golden_from().round_for_display(5));
    println!("  0.618  {}", reading.golden_to().round_for_display(5));
    println!("  0.786  {}", reading.stop().round_for_display(5));
    println!(
        "  price is {:.0}% back{}",
        reading.depth() * 100.0,
        if reading.in_golden_zone(&settings) {
            " — inside the golden zone"
        } else {
            ""
        }
    );

    Ok(())
}

pub fn latest_atr(candles: &[Candle], period: usize) -> Result<Option<PriceDistance>> {
    Ok(atr_series(candles, period)?
        .into_iter()
        .flatten()
        .next_back())
}
