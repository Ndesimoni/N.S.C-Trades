//! Draw the Fibonacci on a real chart, and say where price sits in it.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin fib -- XAU/USD 4h
//!     cargo run -p nsc-work-man --bin fib -- XAU/USD 1h 200
//! ```
//!
//! **TWS or IB Gateway must be running and logged in.**
//!
//! ## Which move it measures
//!
//! The ratios are the easy part. **Which move you measure is the whole game.**
//!
//! **The last two confirmed swings.** That is the move the chart actually
//! made, proved by what price did afterwards rather than by where a window
//! happened to start.
//!
//! When there are not two swings yet it falls back to the highest high and
//! lowest low in the window, and **says which one it used** — because those
//! two answers can be wildly different and a chart that does not say is a
//! chart that lies quietly.

use anyhow::{Context, Result};
use nsc_core::candle::Bar;
use nsc_core::levels::digits_for;
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::IbkrConnection;
use nsc_ta::indicators::fibonacci::{self, Leg};
use nsc_ta::swings::{self, Finder};
use rust_decimal::Decimal;
use std::path::Path;

const RULES: &str = "config/fibonacci.toml";
const SWINGS: &str = "config/swings.toml";

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let mut args = std::env::args().skip(1);
    let symbol = args.next().unwrap_or_else(|| "XAU/USD".into());
    let word = args.next().unwrap_or_else(|| "4h".into());
    let how_many: usize = args.next().and_then(|n| n.parse().ok()).unwrap_or(120);

    let interval = which(&word)?;
    let rules =
        fibonacci::load(Path::new(RULES)).with_context(|| format!("could not read {RULES}"))?;

    let ibkr = IbkrConnection::connect().await?;
    let mut bars = ibkr.candles(&symbol, interval, how_many).await?;
    bars.reverse();

    let swing_rules =
        swings::load(Path::new(SWINGS)).with_context(|| format!("could not read {SWINGS}"))?;

    let (leg, anchored_on) = match from_swings(swing_rules, &bars)? {
        Some(leg) => (
            leg,
            "the last confirmed swing, to where price has got since",
        ),
        None => match biggest_move(&bars) {
            Some(leg) => (leg, "the window's high and low — NOT ENOUGH SWINGS YET"),
            None => {
                println!("\nNo move in that window — every candle is at the same price.");
                return Ok(());
            }
        },
    };

    show(&symbol, interval, &bars, leg, &rules, anchored_on);

    Ok(())
}

/// **From the last confirmed swing to wherever price has got since.**
///
/// This is the leg a trader draws by hand, and the reason it is not simply
/// "the last two confirmed swings" is worth writing down.
///
/// The last two swings are both PROVED, which means the second one is old:
/// nothing confirms until price has given back half of it. On gold's 4-hour
/// that pair was a 41-point move from three weeks ago while price had run 500
/// points since — a perfectly correct answer to a question nobody asked.
///
/// So the anchor is the last confirmed swing and the far end is the extreme
/// price has reached since. **That extreme is not claimed to be a swing** —
/// it has not proved itself and may never — but it is where the move has got
/// to, and every candle in it has closed.
fn from_swings(rules: swings::Rules, bars: &[Bar]) -> Result<Option<Leg>> {
    let found = Finder::over(rules, bars)?;

    let Some(last) = found.last() else {
        return Ok(None);
    };

    // **Everything since that swing, and the move inside it.**
    //
    // Not simply "the extreme in the direction the swing points". A down-leg
    // that has been completely undone is still the last thing that proved
    // itself — on gold's daily that left a June high anchoring a move to
    // 3,941 while price sat 200 points ABOVE the anchor, which is a real
    // answer to a question nobody asked.
    //
    // So the swing sets where to start LOOKING, and the move is whatever the
    // chart has done since: the high and the low of that stretch, and
    // whichever came last is where the move ended.
    let since: Vec<Bar> = bars
        .iter()
        .filter(|bar| bar.opened_at().ok().is_some_and(|at| at >= last.bar_time()))
        .cloned()
        .collect();

    Ok(biggest_move(&since))
}

/// The highest high and the lowest low, in the order they happened.
///
/// **Only used when there are not two swings yet.** Widen the window and this
/// can pick a completely different move — which is the whole reason the old
/// code anchored on confirmed swings instead.
fn biggest_move(bars: &[Bar]) -> Option<Leg> {
    let top = bars.iter().enumerate().max_by_key(|(_, bar)| bar.high)?;
    let bottom = bars.iter().enumerate().min_by_key(|(_, bar)| bar.low)?;

    // Whichever came last is where the move ended.
    if top.0 > bottom.0 {
        Leg::new(bottom.1.low, top.1.high)
    } else {
        Leg::new(top.1.high, bottom.1.low)
    }
}

fn show(
    symbol: &str,
    interval: Interval,
    bars: &[Bar],
    leg: Leg,
    rules: &fibonacci::Rules,
    anchored_on: &str,
) {
    let last = bars.last().expect("a window with candles in it");
    let price = last.close;

    // **Shown to the pair's own precision.** A fib level is a share of a move,
    // so the arithmetic runs long — 4332.89674 on gold is five decimals of
    // false confidence on an instrument quoted to two.
    let digits = digits_for(&symbol.replace('/', ""));
    let show = |value: Decimal| value.round_dp(digits).to_string();

    println!("\n══════ {symbol} · {} ══════\n", interval.spoken());

    println!("  anchored on       {anchored_on}");
    println!(
        "  the move          {} {} → {}",
        if leg.up() { "UP  " } else { "DOWN" },
        show(leg.from()),
        show(leg.to()),
    );
    println!("  price now         {}", show(price));
    println!(
        "  it reads as       {}\n",
        fibonacci::read(leg, price, rules).spoken(),
    );

    println!("  {:<10}{:>14}   what it is for", "share", "price");

    for (share, at) in fibonacci::levels(leg, rules) {
        let job = if share == rules.strong_trend {
            "a reading — the move barely paused"
        } else if share == rules.stop_level {
            "where a stop gets LOOKED at"
        } else {
            "the golden zone"
        };

        let here = if passed(leg, price, at) { " ←" } else { "" };

        println!("  {:<10}{:>14}   {job}{here}", share.to_string(), show(at),);
    }

    println!();

    for (ratio, at) in fibonacci::targets(leg, rules) {
        println!(
            "  {:<10}{:>14}   target — standard number, NOT his",
            ratio.to_string(),
            show(at),
        );
    }

    println!(
        "\n  Read over {} candles. A swing is proved by what price did after it,\n  \
         so the START of the move above is a point the chart PROVED. The far\n  \
         end is where price has got to since, which has NOT proved itself and\n  \
         may never — it is the move in progress, and it is what you would\n  \
         draw by hand.\n",
        bars.len(),
    );
}

/// Has price come back past this level?
fn passed(leg: Leg, price: rust_decimal::Decimal, level: rust_decimal::Decimal) -> bool {
    if leg.up() {
        price <= level
    } else {
        price >= level
    }
}

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
