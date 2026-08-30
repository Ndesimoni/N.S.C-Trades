//! The whole signal — all three pictures, from ONE real signal.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin cards -- bundle
//! ```
//!
//! **No TWS.** It reads `data/history/AUDUSD-1-hour.csv`, which `--bin history`
//! wrote.
//!
//! ## Why this exists
//!
//! It was drawn by hand three times in one afternoon on 30 August 2026, and
//! the third time it sent him **two different pairs in one bundle** — AUD/USD
//! on the charts and gold on the card, because the charts came from saved
//! candles and the card came from the hardcoded gold example next door.
//!
//! The live path cannot do that: `pair` and `timeframe` are bound once and all
//! three renders take those same two values. **The preview could, and did.**
//!
//! So this draws all three from a single `look()`, exactly as the watcher
//! does. A preview that can disagree with itself teaches the wrong thing about
//! code that cannot.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use nsc_core::candle::{Bar, normal_candle};
use nsc_core::levels::{Band, Pair, Timeframe};
use nsc_strategy::{look, reasons};
use nsc_ta::pattern;
use nsc_work_man::places::{OWNER, PREVIEW, PATTERNS, STRATEGY};
use nsc_work_man::{card, telegram};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Where the saved candles are.
const SAVED: &str = "data/history/AUDUSD-1-hour.csv";

/// Enough for the widest picture, plus room to walk forward looking.
const WINDOW: usize = 420;

fn d(text: &str) -> Decimal {
    Decimal::from_str(text).unwrap_or_default()
}

pub async fn bundle(client: &reqwest::Client) -> Result<()> {
    let rules = nsc_strategy::load(Path::new(STRATEGY))?;
    let patterns = pattern::load(Path::new(PATTERNS))?;
    let all = saved().with_context(|| format!("needs {SAVED} — run --bin history"))?;

    // **His own zone, off the 25 August screenshot**, sized the way the bot
    // sizes one rather than typed as a top and a bottom.
    let bands = [Band::around(
        Timeframe::H4,
        d("0.71430"),
        d("0.00104"),
        d("0.55"),
    )];

    let (end, signal) = newest_at_a_zone(&all, &bands, &patterns, &rules)
        .context("no signal at that zone in the saved candles")?;

    let history: Vec<&Bar> = all[end - WINDOW..end].iter().collect();
    let last = |many: usize| -> Vec<&Bar> {
        history[history.len().saturating_sub(many)..].to_vec()
    };

    let pair = Pair {
        symbol: "AUD/USD".into(),
        digits: 5,
        nightly_break_minutes: 60,
        approach_pips: None,
        levels: Vec::new(),
    };
    let timeframe = "1h";

    // **One pair and one timeframe, read once.** Every picture below takes
    // these, so none of them can name something the others do not.
    let pictures = [
        card::render(
            "chart.html",
            &last(400),
            &bands,
            &pair.symbol,
            timeframe,
            pair.digits,
            &PathBuf::from(PREVIEW).join("signal-run.png"),
        )?,
        card::render_ringed(
            "chart.html",
            &last(100),
            &bands,
            &pair.symbol,
            timeframe,
            pair.digits,
            Some(signal.shape.candles()),
            &PathBuf::from(PREVIEW).join("signal-chart.png"),
        )?,
        card::setup(
            &signal,
            &pair,
            &last(signal.shape.candles()),
            timeframe,
            &all[end - 1].datetime,
            &PathBuf::from(PREVIEW).join("setup.png"),
        )?,
    ];

    let words = reasons::sentence(&signal, &pair.symbol, timeframe, pair.digits);
    println!("  {words}");

    let paths: Vec<&Path> = pictures.iter().map(PathBuf::as_path).collect();
    telegram::send_to(client, &OWNER.to_string(), &paths, &words).await?;

    println!("  three pictures sent");
    Ok(())
}

/// The most recent candle in the file that is a signal **at that zone**.
///
/// **At a zone, not merely a signal.** Taking the newest of any kind hands
/// back a `Bold` shape in open water, which is a different tier and not the
/// one worth looking at when checking how a setup is drawn.
fn newest_at_a_zone(
    all: &[Bar],
    bands: &[Band],
    patterns: &pattern::Rules,
    rules: &nsc_strategy::Rules,
) -> Option<(usize, nsc_strategy::Signal)> {
    let mut found = None;

    for end in WINDOW..=all.len() {
        let history: Vec<&Bar> = all[end - WINDOW..end].iter().collect();
        let normal = normal_candle(&history, 14)?;

        if let Some(signal) = look(&history, bands, normal, patterns, rules)
            && signal.standing.band().is_some()
        {
            found = Some((end, signal));
        }
    }

    found
}

/// The saved candles, oldest first.
fn saved() -> Result<Vec<Bar>> {
    let text = std::fs::read_to_string(SAVED)?;

    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 5 {
                return None;
            }

            Some(Bar {
                datetime: parts[0].into(),
                open: parts[1].parse().ok()?,
                high: parts[2].parse().ok()?,
                low: parts[3].parse().ok()?,
                close: parts[4].parse().ok()?,
            })
        })
        .collect())
}
