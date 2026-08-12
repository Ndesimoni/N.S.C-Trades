//! Reads a CSV of candles and prints what the chart-reading code sees.
//!
//! The point of it is to answer one question that no test can: **do the levels
//! it draws match the ones you would draw?**
//!
//! Everything else in this project has only ever run on candles somebody made
//! up. This is the first thing that runs on a real chart.
//!
//!     cargo run -p nsc-backtest --bin chart -- path/to/GBPUSD_D1.csv
//!     cargo run -p nsc-backtest --bin chart -- gold.csv --timeframe H4
//!
//! It changes nothing and writes nothing. It reads a file and prints.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use nsc_core::candle::Candle;
use nsc_core::price::PriceDistance;
use nsc_core::structure::StructureEvent;
use nsc_core::timeframe::Timeframe;
use nsc_data::sources::read_candles;
use nsc_ta::indicators::atr::atr_series;

#[path = "chart/settings.rs"]
mod settings;

use settings::{candle_settings, fib_settings, level_settings, structure_settings, swing_settings};

#[derive(Parser)]
#[command(about = "Print what the chart-reading code sees in a CSV of candles")]
pub struct Args {
    /// The CSV file. Needs a header row with time, open, high, low and close.
    file: PathBuf,

    /// The timeframe the file is in. Only used for tagging levels.
    #[arg(long, default_value = "D1")]
    timeframe: String,

    /// How many candles ATR averages over.
    #[arg(long, default_value_t = 14)]
    atr_period: usize,

    /// How big a run must be next to recent ones. The setting most worth
    /// turning by hand while looking at a chart.
    #[arg(long, default_value_t = 0.5)]
    min_run_fraction: f64,

    /// How far back "recent" reaches, counted in runs.
    #[arg(long, default_value_t = 5)]
    run_memory_legs: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let timeframe: Timeframe = args
        .timeframe
        .parse()
        .with_context(|| format!("'{}' is not a timeframe", args.timeframe))?;

    let candles =
        read_candles(&args.file).with_context(|| format!("reading {}", args.file.display()))?;

    if candles.is_empty() {
        println!("No candles in {}.", args.file.display());
        return Ok(());
    }

    show_the_file(&candles, &args)?;
    show_swings_and_levels(&candles, timeframe, &args)?;
    show_structure(&candles, &args)?;
    show_patterns(&candles, &args)?;
    show_fibonacci(&candles, &args)?;

    println!("\nNothing above is a trading decision. It is what the bot SEES.");
    println!("The last candle may still be forming — the file cannot say.");

    Ok(())
}

fn show_the_file(candles: &[Candle], args: &Args) -> Result<()> {
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

fn show_swings_and_levels(candles: &[Candle], timeframe: Timeframe, args: &Args) -> Result<()> {
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

fn show_structure(candles: &[Candle], args: &Args) -> Result<()> {
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

fn show_patterns(candles: &[Candle], args: &Args) -> Result<()> {
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

fn show_fibonacci(candles: &[Candle], args: &Args) -> Result<()> {
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

fn latest_atr(candles: &[Candle], period: usize) -> Result<Option<PriceDistance>> {
    Ok(atr_series(candles, period)?
        .into_iter()
        .flatten()
        .next_back())
}
