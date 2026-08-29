//! What price actually did after each pattern.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin after -- XAU/USD 4h 1200
//! ```
//!
//! **The base rate is the whole point.** A pattern that is right 55% of the
//! time is worth nothing if price rose 55% of the time anyway — and every
//! candlestick page on the internet quotes the first number without the
//! second.
//!
//! So every pattern is measured against what an ordinary candle did over the
//! same stretch, and what is printed is the DIFFERENCE. Anything inside a
//! point or two of nought is the market, not the pattern.
//!
//! **This is a measurement, not a backtest.** It uses closes, ignores spread
//! and slippage entirely, and says nothing about where a stop would have gone.
//! It belongs in `nsc-backtest` when that crate exists.

mod outcome;

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use nsc_core::candle::{Bar, normal_candle};
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::IbkrConnection;
use nsc_ta::pattern::{self, Pattern};

use outcome::{AHEAD, Record, played_out};

const NORMAL_OVER: usize = 14;
const PATTERNS: &str = "config/patterns.toml";

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let mut args = std::env::args().skip(1);
    let symbol = args.next().unwrap_or_else(|| "XAU/USD".into());
    let word = args.next().unwrap_or_else(|| "4h".into());
    let how_many: usize = args.next().and_then(|n| n.parse().ok()).unwrap_or(1200);

    let interval = which(&word)?;
    let rules =
        pattern::load(Path::new(PATTERNS)).with_context(|| format!("could not read {PATTERNS}"))?;

    let ibkr = IbkrConnection::connect().await?;
    let mut bars = ibkr.candles(&symbol, interval, how_many).await?;
    bars.reverse();

    // **The newest candle is still forming**, and the one before it may be
    // too on a slow chart. Neither belongs in a record of what happened.
    bars.pop();

    report(&symbol, interval, &bars, &rules);

    Ok(())
}

fn report(symbol: &str, interval: Interval, bars: &[Bar], rules: &pattern::Rules) {
    // The record, and which way that pattern claimed — so it can be set
    // against the right control.
    let mut books: BTreeMap<&'static str, ([Record; AHEAD.len()], bool)> = BTreeMap::new();
    // **Two controls, not one.** Gold drifted up this year, so an ordinary
    // candle "claiming up" was right 51% of the time and one claiming down
    // only 49%. Judge a bearish pattern against the up rate and it starts two
    // points behind before it has done anything.
    let mut ordinary = [[Record::default(); AHEAD.len()]; 2];

    for at in NORMAL_OVER..bars.len() {
        let earlier: Vec<&Bar> = bars[at - NORMAL_OVER..at].iter().collect();
        let Some(normal) = normal_candle(&earlier, NORMAL_OVER) else {
            continue;
        };

        // **The control**, kept both ways round.
        for (side, up) in [true, false].into_iter().enumerate() {
            for (which, ahead) in AHEAD.iter().enumerate() {
                if let Some((won, moved)) = played_out(bars, at, *ahead, up, normal) {
                    ordinary[side][which].tried += 1;
                    ordinary[side][which].moved += moved;
                    ordinary[side][which].went_its_way += usize::from(won);
                }
            }
        }

        let so_far: Vec<&Bar> = bars[at.saturating_sub(2)..=at].iter().collect();
        let Some(found) = pattern::ending_at(&so_far, normal, rules) else {
            continue;
        };

        let book = books
            .entry(found.spoken())
            .or_insert(([Record::default(); AHEAD.len()], claims_up(found)));

        for (which, ahead) in AHEAD.iter().enumerate() {
            if let Some((won, moved)) = played_out(bars, at, *ahead, claims_up(found), normal) {
                book.0[which].tried += 1;
                book.0[which].moved += moved;
                book.0[which].went_its_way += usize::from(won);
            }
        }
    }

    print(symbol, interval, bars.len(), &books, &ordinary);
}

/// How far a coin would stray on this many flips — one standard error.
///
/// **Without this the table is a Rorschach test.** On 200 tries a fair coin
/// lands 3.5 points either side of 50% as a matter of course, so a pattern
/// "beating the market by 3" is a pattern doing nothing. And with fourteen
/// patterns at four horizons there are fifty-six numbers here: a third of
/// them will clear one standard error by luck.
fn noise(tried: usize) -> f64 {
    if tried == 0 {
        return 0.0;
    }

    (0.25f64 / tried as f64).sqrt() * 100.0
}

/// Which way each pattern says price should go.
fn claims_up(found: Pattern) -> bool {
    match found {
        Pattern::Engulfing { up }
        | Pattern::Harami { up }
        | Pattern::Star { up, .. }
        | Pattern::Marching { up }
        | Pattern::Push { up } => up,
        Pattern::Tweezer { top } => !top,
        Pattern::PiercingLine => true,
        Pattern::DarkCloudCover => false,
    }
}

fn print(
    symbol: &str,
    interval: Interval,
    read: usize,
    books: &BTreeMap<&'static str, ([Record; AHEAD.len()], bool)>,
    ordinary: &[[Record; AHEAD.len()]; 2],
) {
    println!(
        "\n══════ {symbol} · {} · {read} candles ══════\n",
        interval.spoken()
    );

    print!("  {:<22}{:>6}{:>8}", "", "found", "noise");
    for ahead in AHEAD {
        print!("{:>15}", format!("{ahead} ahead"));
    }
    println!("\n");

    for (side, label) in [(0, "ORDINARY, CLAIMING UP"), (1, "ORDINARY, CLAIMING DOWN")] {
        print!("  {label:<22}{:>6}{:>8}", ordinary[side][0].tried, "");

        for one in &ordinary[side] {
            print!("{:>15}", format!("{:.1}%", one.rate()));
        }

        println!();
    }

    println!("  {:-<88}", "");

    let mut rows: Vec<_> = books.iter().collect();
    rows.sort_by_key(|(_, book)| std::cmp::Reverse(book.0[0].tried));

    for (name, (book, up)) in rows {
        let side = usize::from(!*up);

        print!(
            "  {name:<22}{:>6}{:>8}",
            book[0].tried,
            format!("±{:.1}", noise(book[0].tried)),
        );

        for (which, one) in book.iter().enumerate() {
            print!(
                "{:>15}",
                format!("{:+.1}", one.rate() - ordinary[side][which].rate())
            );
        }

        println!();
    }

    println!("\n  Each number is POINTS BETTER OR WORSE than an ordinary candle over");
    println!("  the same stretch.");
    println!();
    println!("  NOISE is how far a coin would stray on that many flips — one standard");
    println!("  error. A number smaller than its own noise column says nothing at all,");
    println!("  and about a third of them will clear it by luck alone.");
    println!();
    println!(
        "  There are {} numbers in that table. Some will look striking.",
        books.len() * AHEAD.len()
    );
    println!();

    edges(books, ordinary);
}

/// The other half: how far it went, not only how often.
fn edges(
    books: &BTreeMap<&'static str, ([Record; AHEAD.len()], bool)>,
    ordinary: &[[Record; AHEAD.len()]; 2],
) {
    println!("── average move 5 candles on, in ATR, the way the pattern claimed ──\n");

    for (side, label) in [(0, "ORDINARY, CLAIMING UP"), (1, "ORDINARY, CLAIMING DOWN")] {
        println!("  {label:<24}{:>10}", ordinary[side][2].edge().to_string());
    }

    println!();

    // Each one against the control facing ITS way, not always the up one.
    let against = |(book, up): &([Record; AHEAD.len()], bool)| {
        book[2].edge() - ordinary[usize::from(!*up)][2].edge()
    };

    let mut rows: Vec<_> = books.iter().collect();
    rows.sort_by(|a, b| against(b.1).cmp(&against(a.1)));

    for (name, book) in rows {
        println!("  {name:<24}{:>10}", against(book).to_string());
    }

    println!("\n  A win rate can lie in both directions — nine small wins and one");
    println!("  enormous loss is a 90% pattern that loses money. This is the other");
    println!("  half of the answer.\n");
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
