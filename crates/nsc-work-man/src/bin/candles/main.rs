//! Ask IBKR where it starts its day, and check the candles against his chart.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin candles              XAU/USD
//!     cargo run -p nsc-work-man --bin candles -- EUR/USD
//! ```
//!
//! **TWS or IB Gateway must be running and logged in.**
//!
//! Two questions, and the second is the dangerous one.
//!
//! ARE THE NUMBERS RIGHT? Put the highs and lows below next to his chart.
//!
//! WHERE DOES IBKR START ITS DAY? `config/when.toml` says 17:00 New York, and
//! that was measured on the OLD feed. If IBKR ends its day somewhere else,
//! every daily candle is a different candle — different open, different high,
//! different range. Band thickness is 0.46 of a normal daily candle, so every
//! daily band changes size and every daily level moves. **Nothing errors.**

mod boundary;

use anyhow::Result;
use nsc_core::candle::Bar;
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::{IbkrConnection, Serves};

/// How many big candles to test a boundary with.
///
/// **Six, because one match is a coincidence.** A quiet market opens two hours
/// on the same number often enough that a single agreement proves nothing.
const TESTED: usize = 6;

/// How many to print. Enough to hold next to a chart, not enough to scroll.
const SHOWN: usize = 8;

/// Enough hourly candles to cover those six days with room at the edges.
const HOURS: usize = 200;

/// Enough daily candles to cover the weeks being tested.
///
/// **This is the one that was wrong.** It was six — the same number as the
/// days being tested — so the weekly check lined five weekly candles up
/// against barely one week of days. Four of them could never match, and it
/// would have reported NOT SETTLED: a wrong answer wearing a finding's
/// clothes.
///
/// Five trading days a week, six weeks tested, plus room for holidays.
const DAYS: usize = 40;

/// Enough weeks to test the weekly boundary the same way.
const WEEKS: usize = TESTED;

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let symbol = std::env::args().nth(1).unwrap_or_else(|| "XAU/USD".into());
    let ibkr = IbkrConnection::connect().await?;

    // **Before measuring anything: does IBKR know this instrument at all?**
    // Measuring a pair it has never heard of produces an empty answer that
    // reads like a quiet market.
    match ibkr.serves(&symbol).await? {
        Serves::Yes => println!("\nIBKR knows {symbol}."),
        Serves::Never { why } => {
            println!("\n✗ IBKR does not know {symbol} — {why}");
            return Ok(());
        }
    }

    let hourly = ibkr.candles(&symbol, Interval::H1, HOURS).await?;
    let daily = ibkr.candles(&symbol, Interval::Day, DAYS).await?;
    let weekly = ibkr.candles(&symbol, Interval::Week, WEEKS).await?;

    println!("\n══════ {symbol} ══════\n");

    show("the last daily candles", &daily);
    show("the last weekly candles", &weekly);

    // **Each boundary is tested against the candles one step below it.** The
    // day against hours, the week against days — and each needs enough of the
    // smaller ones to cover every big candle being tested, or a candle that
    // simply fell outside the sample reads as a candle that did not match.
    measure(
        "THE DAY",
        "hour",
        &daily[..TESTED.min(daily.len())],
        &hourly,
        boundary::hour_of,
    );

    // **The week is measured in DAYS, not hours.** A daily candle carries a
    // date and no time, so every stamp reads 00:00 — and reporting that would
    // be a fact about the stamp rather than about IBKR.
    measure("THE WEEK", "day", &weekly, &daily, boundary::weekday_of);

    Ok(())
}

/// The candles as they came back, for holding next to his chart.
///
/// Only the newest few. Forty daily candles is a screen of scrolling, and the
/// ones worth putting next to his chart are the recent ones.
fn show(what: &str, bars: &[Bar]) {
    let bars = &bars[..SHOWN.min(bars.len())];

    println!("{what} — newest first\n");
    println!(
        "    {:<21}{:>12}{:>12}{:>12}{:>12}",
        "stamp", "open", "high", "low", "close"
    );

    for bar in bars {
        println!(
            "    {:<21}{:>12}{:>12}{:>12}{:>12}",
            bar.datetime, bar.open, bar.high, bar.low, bar.close
        );
    }

    println!();
}

/// How many candles must vote before an answer is worth writing down.
///
/// **Two agreeing is not a measurement.** On EUR/USD exactly two of six could
/// vote — the rest opened on a price two hourly candles shared — and the
/// first version called that EVERY CANDLE AGREES.
const QUORUM: usize = 3;

/// Line the big candles up against the small ones and say what it found.
fn measure(
    what: &str,
    unit: &str,
    big: &[Bar],
    small: &[Bar],
    speak: impl Fn(&str) -> Option<String> + Copy,
) {
    println!("── WHERE IBKR STARTS {what} ──\n");
    println!("  Each candle's open is also a smaller candle's open — the same tick,");
    println!("  written twice. The {unit} that shares the number is where it began.\n");

    let lined = boundary::line_up(big, small);

    for one in &lined {
        match one.started.len() {
            1 => println!("    {}   started {}", one.big, one.started[0]),
            0 => match &one.nearest {
                Some((when, off)) => {
                    println!("    {}   NO MATCH — nearest {when}, out by {off}", one.big)
                }
                None => println!("    {}   NO MATCH — nothing to compare against", one.big),
            },
            many => println!(
                "    {}   {many} of them share that open — settles nothing",
                one.big
            ),
        }
    }

    println!();

    let voted = boundary::voted(&lined);
    let agreed = boundary::agreed_on(&lined, speak);

    println!("  {voted} of {} candles could vote.\n", lined.len());

    match agreed {
        Some(answer) if voted >= QUORUM => {
            println!("  ► ALL {voted} AGREE: {what} starts {answer} UTC.\n");
            println!("  Check it against config/when.toml, which says the day ends 17:00");
            println!("  New York — 21:00 UTC in summer, 22:00 in winter. Anything else");
            println!("  and every daily and weekly band is sized off the wrong candle,");
            println!("  and nothing will ever error to say so.\n");
        }

        // **Agreeing is not the same as being enough.** A candle whose open is
        // shared by two smaller ones has no vote — it is silent, not evidence.
        Some(answer) => {
            println!("  ► NOT SETTLED. The {voted} that voted all said {answer}, and");
            println!("  {QUORUM} is the least worth writing down. Run it again while the");
            println!("  market is open, or on a busier pair.\n");
        }

        None => {
            println!("  ► NOT SETTLED. The candles do not agree on one {unit}.\n");
            println!("  Do not write a boundary into config/when.toml from this. A shut");
            println!("  market gives thin candles that share opens by accident.\n");
        }
    }
}
