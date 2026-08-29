//! Pull years of candles once, and keep them on disk.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin history
//!     cargo run -p nsc-work-man --bin history -- XAU/USD 4h
//! ```
//!
//! **TWS or IB Gateway has to be running.**
//!
//! **Fetched once, read many times.** Working on a detector means running it
//! over the same three years fifty times, and asking IBKR fifty times for
//! candles that have not changed is both slow and rude — sixty requests in ten
//! minutes is the limit, and it paces rather than refusing when you go over.
//!
//! **This is not the Postgres backfill.** That is designed in
//! `docs/worksheets/database.md` and does not exist. This is a file per pair
//! and timeframe, so a detector can be worked on before the database lands.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use nsc_core::levels;
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::IbkrConnection;
use nsc_work_man::places::PAIRS;
use nsc_work_man::retry::keep_trying;

/// Where the files go.
pub const HISTORY: &str = "data/history";

/// The two he executes on, and the two the chart patterns are hunted on.
const PULL: [Interval; 2] = [Interval::H1, Interval::H4];

/// How many to ask for.
///
/// **More than three years, deliberately.** IBKR serves what it has and stops,
/// so asking for too many costs nothing and asking for too few silently gives
/// a shorter history than the one you thought you had. Measured on 27 August:
/// 20,000 asked on the 1-hour returned 28,865 going back to April 2023.
const ASK_FOR: usize = 30_000;

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let mut args = std::env::args().skip(1);
    let only = args.next();
    let only_interval = args.next();

    let ibkr = IbkrConnection::connect().await?;
    let folder = Path::new(PAIRS);

    std::fs::create_dir_all(HISTORY).context("could not make data/history")?;

    for name in levels::known(folder) {
        let Ok(pair) = levels::load_pair(&folder.join(format!("{name}.toml"))) else {
            continue;
        };

        if let Some(wanted) = &only
            && !pair.symbol.eq_ignore_ascii_case(wanted)
            && !name.eq_ignore_ascii_case(wanted)
        {
            continue;
        }

        for interval in PULL {
            if let Some(wanted) = &only_interval
                && !interval.spoken().eq_ignore_ascii_case(wanted)
            {
                continue;
            }

            if let Err(trouble) = one(&ibkr, &pair.symbol, interval).await {
                eprintln!("{} {}: {trouble:#}", pair.symbol, interval.spoken());
            }
        }
    }

    Ok(())
}

/// One pair, one timeframe, written to its own file.
async fn one(ibkr: &IbkrConnection, symbol: &str, interval: Interval) -> Result<()> {
    let mut bars = keep_trying(3, || ibkr.candles(symbol, interval, ASK_FOR))
        .await
        .context("could not fetch")?;

    // Newest first from the feed. Oldest first is how everything reads, and it
    // is how a file wants to be written.
    bars.reverse();

    let path = file_for(symbol, interval);

    // **Written as text, one candle a line.** Not because a format was needed
    // but because this file gets looked at by eye when a detector says
    // something surprising, and a binary one cannot be.
    let mut out = String::with_capacity(bars.len() * 64);
    out.push_str("opened_at,open,high,low,close\n");

    for bar in &bars {
        out.push_str(&format!(
            "{},{},{},{},{}\n",
            bar.datetime, bar.open, bar.high, bar.low, bar.close
        ));
    }

    std::fs::write(&path, out).with_context(|| format!("could not write {}", path.display()))?;

    let first = bars.first().map_or("—", |bar| bar.datetime.as_str());
    let last = bars.last().map_or("—", |bar| bar.datetime.as_str());

    println!(
        "  {:9} {:6} {:>7} candles   {}  ->  {}",
        symbol,
        interval.spoken(),
        bars.len(),
        first,
        last
    );

    Ok(())
}

/// `data/history/XAUUSD-4h.csv`
pub fn file_for(symbol: &str, interval: Interval) -> PathBuf {
    let plain: String = symbol.chars().filter(char::is_ascii_alphanumeric).collect();

    PathBuf::from(HISTORY).join(format!("{plain}-{}.csv", interval.spoken()))
}
