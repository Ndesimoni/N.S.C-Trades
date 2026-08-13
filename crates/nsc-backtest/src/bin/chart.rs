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
use chrono::Weekday;
use chrono_tz::Tz;
use clap::Parser;
use nsc_core::timeframe::{DayBoundary, Timeframe};
use nsc_data::sources::read_candles;

#[path = "chart/settings.rs"]
mod settings;

#[path = "chart/as_json.rs"]
mod as_json;

#[path = "chart/printing.rs"]
mod printing;

use as_json::as_json;
use printing::{
    show_fibonacci, show_patterns, show_structure, show_swings_and_levels, show_the_file,
};

#[derive(Parser)]
#[command(about = "Print what the chart-reading code sees in a CSV of candles")]
pub struct Args {
    /// The CSV file. Needs a header row with time, open, high, low and close.
    file: PathBuf,

    /// The timeframe the file is in.
    #[arg(long, default_value = "D1")]
    from: String,

    /// The timeframe to read the chart at. If it is bigger than `--from`, the
    /// candles are built up to it first, through the same aggregator the live
    /// bot uses.
    #[arg(long)]
    timeframe: Option<String>,

    /// Read the levels the trader drew himself, instead of finding them.
    ///
    /// Point it at `config/levels/XAUUSD.toml`. These are what the bot
    /// actually trades — the finder only exists to be scored against them.
    #[arg(long)]
    pub levels: Option<PathBuf>,

    /// Print the findings as JSON instead, for drawing elsewhere.
    #[arg(long, default_value_t = false)]
    json: bool,

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

    /// How thick a level band is, as a fraction of a normal candle.
    #[arg(long, default_value_t = 0.5)]
    pub band_atr: f64,

    /// How many touches before a band counts as a level.
    #[arg(long, default_value_t = 2)]
    pub min_touches: usize,

    /// How far back to look for levels, in candles.
    #[arg(long, default_value_t = 500)]
    pub max_age: usize,

    /// How far apart two levels on the SAME timeframe must sit, in bands.
    #[arg(long, default_value_t = 3.0)]
    pub min_separation: f64,

    /// How clear of a bigger level a smaller one must sit, in the bigger
    /// level's bands.
    #[arg(long, default_value_t = 1.5)]
    pub absorb_gap: f64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let from: Timeframe = args
        .from
        .parse()
        .with_context(|| format!("'{}' is not a timeframe", args.from))?;

    let timeframe: Timeframe = match &args.timeframe {
        Some(text) => text
            .parse()
            .with_context(|| format!("'{text}' is not a timeframe"))?,
        None => from,
    };

    let file =
        read_candles(&args.file).with_context(|| format!("reading {}", args.file.display()))?;

    if file.is_empty() {
        println!("No candles in {}.", args.file.display());
        return Ok(());
    }

    // Built through the same aggregator the live bot uses, so what gets read
    // here is what the bot would have read.
    let candles = if timeframe == from {
        file
    } else {
        nsc_ta::aggregate::aggregate(&file, from, timeframe, &boundary()?)?
    };

    if candles.is_empty() {
        println!("Not enough {from} candles to build one {timeframe}.");
        return Ok(());
    }

    if args.json {
        return as_json(&candles, timeframe, &args);
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

/// The daily close this project uses, from config/app.toml.
pub fn boundary() -> Result<DayBoundary> {
    let tz: Tz = "America/New_York"
        .parse()
        .map_err(|_| anyhow::anyhow!("America/New_York is not a timezone this build knows"))?;

    Ok(DayBoundary::new(17, 0, tz, Weekday::Sun)?)
}
