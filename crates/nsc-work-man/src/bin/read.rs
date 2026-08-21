//! Read a chart the way the code reads it, and print what it found.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin read                      XAU/USD 4h
//!     cargo run -p nsc-work-man --bin read -- EUR/USD 1h
//!     cargo run -p nsc-work-man --bin read -- XAU/USD 1d 120
//! ```
//!
//! **TWS or IB Gateway must be running and logged in.**
//!
//! **Nothing here is an opinion.** It fetches real candles, runs `nsc-ta` over
//! them, and prints the name the CODE gave each one. If a shape looks wrong on
//! the chart, the thresholds in `config/candles.toml` are wrong — not the
//! reading.
//!
//! That is the whole point of it. Asking anyone, person or model, to eyeball a
//! chart and name what they see gets you a confident, believable answer that
//! nothing checked.

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::candle::{Bar, normal_candle};
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::{IbkrConnection, Serves};
use nsc_ta::candle::Shape;
use nsc_ta::pattern;
use std::collections::BTreeMap;
use std::path::Path;

/// How many candles a "normal" one is averaged over. Fourteen is the usual.
const NORMAL_OVER: usize = 14;

/// How many to read, unless he says otherwise.
const HOW_MANY: usize = 60;

/// Where the naming thresholds live.
const RULES: &str = "config/candles.toml";

/// And the ones for a run of candles.
const PATTERNS: &str = "config/patterns.toml";

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let mut args = std::env::args().skip(1);
    let symbol = args.next().unwrap_or_else(|| "XAU/USD".into());
    let interval = which(&args.next().unwrap_or_else(|| "4h".into()))?;
    let how_many = args.next().and_then(|n| n.parse().ok()).unwrap_or(HOW_MANY);

    let rules = nsc_ta::candle::load(Path::new(RULES))
        .with_context(|| format!("could not read {RULES}"))?;

    let runs =
        pattern::load(Path::new(PATTERNS)).with_context(|| format!("could not read {PATTERNS}"))?;

    let ibkr = IbkrConnection::connect().await?;

    if let Serves::Never { why } = ibkr.serves(&symbol).await? {
        println!("\n✗ IBKR does not know {symbol} — {why}");
        return Ok(());
    }

    // Newest first from the feed; a chart is read the other way.
    let mut bars = ibkr
        .candles(&symbol, interval, how_many + NORMAL_OVER)
        .await?;
    bars.reverse();

    show(&symbol, interval, &bars, &rules, &runs, how_many);

    Ok(())
}

/// The candles, each with the name the code gave it.
fn show(
    symbol: &str,
    interval: Interval,
    bars: &[Bar],
    rules: &nsc_ta::candle::Rules,
    runs: &pattern::Rules,
    how_many: usize,
) {
    println!("\n══════ {symbol} · {} ══════\n", interval.spoken());
    println!(
        "  {:<20}{:>11}{:>11}{:>11}{:>11}{:>8}   the code calls it",
        "opened", "open", "high", "low", "close", "reach"
    );

    let mut tally: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut runs_found: Vec<(String, &'static str)> = Vec::new();
    let start = bars.len().saturating_sub(how_many).max(NORMAL_OVER);

    for (at, bar) in bars.iter().enumerate().skip(start) {
        // **A normal candle is measured from what came BEFORE this one.**
        // Including the candle being judged, or anything after it, is reading
        // price the market had not printed yet — and it does not error, it
        // just makes every reach look more ordinary than it was.
        let earlier: Vec<&Bar> = bars[at.saturating_sub(NORMAL_OVER)..at].iter().collect();

        let Some(normal) = normal_candle(&earlier, NORMAL_OVER) else {
            continue;
        };

        // A flat holiday candle has no shape, and that is not a fault.
        let Some(shape) = Shape::of(bar, normal) else {
            println!("  {:<20}{:>11}   flat — no shape", bar.datetime, bar.open);
            continue;
        };

        let called = shape.named(rules);

        // **The newest candle is usually still running, and its shape is not
        // its shape yet.** A doji at ten past the hour is a doji that has not
        // happened — the body it ends with is nowhere on the screen. It is
        // shown because he asked what is on the chart, and it is MARKED
        // because counting it as a finished shape is reading price the market
        // has not printed.
        //
        // Asked of the clock, never of position in the list.
        let finished = bar
            .finished_by(Utc::now(), interval.minutes())
            .unwrap_or(false);

        if finished {
            *tally.entry(called.spoken()).or_default() += 1;

            // **Only what came BEFORE, and only up to this candle.**
            // `ending_at` looks backwards from the last bar it is handed, so
            // there is no way to reach a candle the market had not printed.
            //
            // And only once this one has CLOSED. A pattern that ends on a
            // candle still being drawn is not a pattern yet — its last body
            // is nowhere on the screen, and it can un-happen before the hour
            // is out.
            let so_far: Vec<&Bar> = bars[at.saturating_sub(2)..=at].iter().collect();

            if let Some(run) = pattern::ending_at(&so_far, normal, runs) {
                runs_found.push((bar.datetime.clone(), run.spoken()));
            }
        }

        println!(
            "  {:<20}{:>11}{:>11}{:>11}{:>11}{:>7}x   {}",
            bar.datetime,
            bar.open,
            bar.high,
            bar.low,
            bar.close,
            shape.reach.round_dp(1),
            if finished {
                called.spoken().to_string()
            } else {
                format!("{} — STILL FORMING, not counted", called.spoken())
            },
        );
    }

    counted(&tally);
    runs_seen(&runs_found);
}

/// What turned up, commonest first.
///
/// **This is the part worth reading.** A shape that fills half the column is
/// describing the market, not marking it — and a rule built on one would fire
/// every day and mean nothing.
fn counted(tally: &BTreeMap<&'static str, usize>) {
    let mut rows: Vec<(&&str, &usize)> = tally.iter().collect();
    rows.sort_by(|a, b| b.1.cmp(a.1));

    let total: usize = tally.values().sum();

    println!("\n── what turned up in {total} candles ──\n");

    for (name, found) in rows {
        let share = found * 100 / total.max(1);
        let bar = "█".repeat((share / 3).max(1));

        println!("  {name:<20}{found:>4}  {share:>3}%  {bar}");
    }

    println!(
        "\n  A shape is a description, not a signal. What makes one worth\n  \
         anything is the level it printed at — and that is not this crate's\n  \
         question.\n\n  \
         NO TREND HERE. Swings are not built, so nothing in this project can\n  \
         yet say which way the chart is going. A shape that needs the trend to\n  \
         be named — a hammer against a hanging man — is left unnamed on\n  \
         purpose.\n"
    );
}

/// The patterns that finished in the window.
///
/// **A pattern is a description too.** It says two or three candles did a
/// thing together, not that the thing means anything — that depends on the
/// level it printed at, and this does not know one.
fn runs_seen(found: &[(String, &'static str)]) {
    println!("── patterns that finished ──\n");

    if found.is_empty() {
        println!("  None in this window.\n");
        return;
    }

    for (stamp, called) in found {
        println!("  {stamp}   {called}");
    }

    println!();
}

/// The timeframe he asked for.
fn which(word: &str) -> Result<Interval> {
    Ok(match word.to_lowercase().as_str() {
        "5m" | "5min" => Interval::Min5,
        "15m" | "15min" => Interval::Min15,
        "30m" | "30min" => Interval::Min30,
        "1h" | "h1" => Interval::H1,
        "4h" | "h4" => Interval::H4,
        "1d" | "d1" | "day" | "daily" => Interval::Day,
        "1w" | "w1" | "week" | "weekly" => Interval::Week,
        other => anyhow::bail!("'{other}' is not a timeframe — try 1h, 4h, 1d or 1w"),
    })
}
