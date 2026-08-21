//! Find the swings on a real chart, and say when each one became knowable.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin swings -- XAU/USD 4h 300
//! ```
//!
//! **TWS or IB Gateway must be running and logged in.**
//!
//! The column worth reading is `known after`. A swing is not knowable on the
//! candle it sits on — you need candles afterwards to prove a peak was a peak
//! — and how many is not fixed. Sometimes two, sometimes thirty.
//!
//! That is the honest answer and it is the whole reason the old three-candle
//! rule was thrown out.

use anyhow::{Context, Result};
use nsc_core::candle::Bar;
use nsc_core::levels::digits_for;
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::IbkrConnection;
use nsc_ta::swings::{self, Finder};
use std::path::Path;

const RULES: &str = "config/swings.toml";

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let mut args = std::env::args().skip(1);
    let symbol = args.next().unwrap_or_else(|| "XAU/USD".into());
    let word = args.next().unwrap_or_else(|| "4h".into());
    let how_many: usize = args.next().and_then(|n| n.parse().ok()).unwrap_or(300);

    let interval = which(&word)?;
    let rules =
        swings::load(Path::new(RULES)).with_context(|| format!("could not read {RULES}"))?;

    let ibkr = IbkrConnection::connect().await?;
    let mut bars = ibkr.candles(&symbol, interval, how_many).await?;
    bars.reverse();

    // **The newest candle is still forming.** A swing proved by a candle that
    // has not closed is a swing that can un-prove itself.
    bars.pop();

    show(&symbol, interval, &bars, rules)?;

    Ok(())
}

fn show(symbol: &str, interval: Interval, bars: &[Bar], rules: swings::Rules) -> Result<()> {
    let digits = digits_for(&symbol.replace('/', ""));
    let found = Finder::over(rules, bars)?;

    println!(
        "\n══════ {symbol} · {} · {} candles ══════\n",
        interval.spoken(),
        bars.len()
    );

    if found.is_empty() {
        println!("  No swing proved itself in that window.\n");
        return Ok(());
    }

    println!(
        "  {:<22}{:>12}   {:<12}known after",
        "sits on", "price", "kind"
    );

    let mut waits: Vec<i64> = Vec::new();

    for swing in &found {
        let at = bars
            .iter()
            .position(|b| b.opened_at().ok() == Some(swing.bar_time()));
        let by = bars
            .iter()
            .position(|b| b.opened_at().ok() == Some(swing.confirmed_at()));

        let waited = match (at, by) {
            (Some(a), Some(b)) => {
                waits.push((b - a) as i64);
                format!("{} candle{}", b - a, if b - a == 1 { "" } else { "s" })
            }
            _ => "—".into(),
        };

        println!(
            "  {:<22}{:>12}   {:<12}{waited}",
            swing.bar_time().format("%Y-%m-%d %H:%M").to_string(),
            swing.price().round_dp(digits).to_string(),
            swing.kind().spoken(),
        );
    }

    counted(&found, &waits, bars.len());

    Ok(())
}

fn counted(found: &[nsc_core::swing::Swing], waits: &[i64], read: usize) {
    let highs = found
        .iter()
        .filter(|s| s.kind() == nsc_core::swing::SwingKind::High)
        .count();

    println!("\n── {} swings in {read} candles ──\n", found.len());
    println!("  {highs} highs, {} lows", found.len() - highs);

    if waits.is_empty() {
        return;
    }

    let mut sorted = waits.to_vec();
    sorted.sort_unstable();

    let total: i64 = sorted.iter().sum();

    println!(
        "\n  known after: {} at best, {} at worst, {} on average (candles)",
        sorted[0],
        sorted[sorted.len() - 1],
        total / sorted.len() as i64,
    );

    println!(
        "\n  NOT A FIXED NUMBER, and that is the point. The old rule said three\n  \
         candles, always. A swing is knowable at the candle where the pullback\n  \
         proved it — and on a slow drift that can be weeks.\n"
    );
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
