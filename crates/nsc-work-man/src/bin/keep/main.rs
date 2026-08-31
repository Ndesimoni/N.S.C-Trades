//! Put the saved candles into Postgres, and read them back.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin keep
//! ```
//!
//! **No TWS.** It reads `data/history/*.csv`, which `--bin history` wrote, and
//! writes them to the record. Running it twice repairs rather than duplicates.

use anyhow::{Context, Result};
use nsc_core::candle::Bar;
use nsc_data::source::Interval;
use nsc_data::store;
use std::path::{Path, PathBuf};

fn read(path: &Path) -> Result<Vec<Bar>> {
    let text = std::fs::read_to_string(path)?;
    let mut out = Vec::new();

    for line in text.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 5 {
            continue;
        }
        out.push(Bar {
            datetime: parts[0].into(),
            open: parts[1].parse()?,
            high: parts[2].parse()?,
            low: parts[3].parse()?,
            close: parts[4].parse()?,
        });
    }

    Ok(out)
}

/// `AUDUSD-1-hour` -> `AUD/USD` and the timeframe.
fn named(stem: &str) -> Option<(String, Interval)> {
    let (pair, rest) = stem.split_once('-')?;
    let interval = match rest {
        "1-hour" => Interval::H1,
        "4-hour" => Interval::H4,
        "daily" => Interval::Day,
        "weekly" => Interval::Week,
        _ => return None,
    };

    let symbol = if pair.len() == 6 {
        let (left, right) = pair.split_at(3);
        format!("{left}/{right}")
    } else {
        pair.to_string()
    };

    Some((symbol, interval))
}

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let url =
        std::env::var("DATABASE_URL").context("DATABASE_URL is not set — see .env.example")?;

    let db = store::open(&url)
        .await
        .context("could not open the record — is `docker compose up -d` running?")?;

    println!("  connected, migrations up to date\n");

    let mut files: Vec<PathBuf> = std::fs::read_dir("data/history")
        .context("no data/history — run --bin history first")?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|kind| kind == "csv"))
        .collect();
    files.sort();

    for path in &files {
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let Some((symbol, interval)) = named(&stem) else {
            println!("  {stem}: not a name I know, skipped");
            continue;
        };

        let bars = read(path).with_context(|| format!("reading {stem}"))?;
        let written = store::write(&db, &symbol, interval, &bars).await?;

        let held = store::count(&db, &symbol, interval).await?;
        let from = store::oldest(&db, &symbol, interval).await?;
        let to = store::newest(&db, &symbol, interval).await?;

        println!(
            "  {:9} {:3}  {:>6} written · {:>6} held  {}  ->  {}",
            symbol,
            interval.stored(),
            written,
            held,
            from.map_or("—".into(), |at| at.format("%Y-%m-%d").to_string()),
            to.map_or("—".into(), |at| at.format("%Y-%m-%d").to_string()),
        );
    }

    Ok(())
}
