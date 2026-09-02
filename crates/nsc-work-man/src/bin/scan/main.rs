//! Look at every pair, on the 1-hour and the 4-hour, and say what is at his
//! zones right now.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin scan
//!     cargo run -p nsc-work-man --bin scan -- send     ...and put it on Telegram
//! ```
//!
//! **TWS or IB Gateway has to be running.**
//!
//! **This is the bot's own question, asked on demand.** The watcher only looks
//! when price is already at a zone; this looks everywhere, at once, and says
//! what it found.
//!
//! It runs `nsc_strategy::look` — the same rules, the same thresholds, the same
//! `config/strategy.toml`. **Nothing here has an opinion of its own**, which is
//! the whole reason it is worth reading.

mod report;

use std::path::Path;

use anyhow::{Context, Result};
use nsc_core::candle::{Bar, normal_candle};
use nsc_core::levels::{self, Band, Pair};
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::IbkrConnection;
use nsc_strategy::Signal;
use nsc_ta::pattern::{self, Pattern};
use nsc_work_man::places::{PAIRS, PATTERNS, STRATEGY, THICKNESS};
use nsc_work_man::retry::keep_trying;

/// The two he executes on. The same list `closes/fetch.rs` reports on.
const LOOK_AT: [Interval; 2] = [Interval::H1, Interval::H4];

/// How far back to look for a shape.
///
/// **Twenty is the fewest that can answer the question honestly.** A normal
/// candle is an average over fourteen, and a shape is judged against how big a
/// normal candle was at that moment — so fourteen of these are spent before
/// the first shape can be judged at all.
const HISTORY: usize = 40;

/// What one pair and interval turned up.
pub struct Found {
    pub symbol: String,
    pub interval: Interval,
    pub digits: u32,

    /// The newest candle, and what the code calls it.
    pub last: Bar,

    /// Every shape in the window, with the candle that completed it.
    pub shapes: Vec<(String, String)>,

    /// The ones that are also at a zone. **This is the answer to his question.**
    pub at_zones: Vec<Signal>,

    /// His zones on this pair, so the report can say what was watched.
    pub bands: Vec<Band>,
}

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let sending = std::env::args().any(|word| word == "send");

    let thickness = levels::load_thickness(Path::new(THICKNESS))?;
    let rules = nsc_strategy::load(Path::new(STRATEGY))
        .with_context(|| format!("could not read {STRATEGY}"))?;
    let named =
        pattern::load(Path::new(PATTERNS)).with_context(|| format!("could not read {PATTERNS}"))?;

    let ibkr = IbkrConnection::connect().await?;
    let folder = Path::new(PAIRS);

    let mut found = Vec::new();

    for name in levels::known(folder) {
        let Ok(pair) = levels::load_pair(&folder.join(format!("{name}.toml"))) else {
            eprintln!("{name}: its file will not read");
            continue;
        };

        // Sized exactly the way the bot sizes them at startup — same function,
        // so the zones on this report and the zones it watches cannot differ.
        let bands = match nsc_work_man::watch::size_bands(&ibkr, &pair, thickness).await {
            Ok(bands) => bands,
            Err(trouble) => {
                eprintln!("{}: could not size its bands — {trouble:#}", pair.symbol);
                continue;
            }
        };

        for interval in LOOK_AT {
            match one(&ibkr, &pair, &bands, interval, &named, &rules).await {
                Ok(seen) => found.push(seen),
                Err(trouble) => eprintln!("{} {}: {trouble:#}", pair.symbol, interval.spoken()),
            }
        }
    }

    report::to_terminal(&found);

    if sending {
        report::to_telegram(&found).await?;
    } else {
        println!("\nRun it with `-- send` to put this on Telegram.");
    }

    Ok(())
}

/// One pair, one interval.
async fn one(
    ibkr: &IbkrConnection,
    pair: &Pair,
    bands: &[Band],
    interval: Interval,
    named: &pattern::Rules,
    rules: &nsc_strategy::Rules,
) -> Result<Found> {
    let mut bars = keep_trying(3, || ibkr.candles(&pair.symbol, interval, HISTORY))
        .await
        .context("could not fetch the candles")?;

    // Newest first from the feed; oldest first is how everything reads.
    bars.reverse();

    // **The newest is still forming.** A shape on a candle that has not closed
    // is not a shape — it can un-form before the close.
    bars.pop();

    let last = bars
        .last()
        .context("no finished candles came back")?
        .clone();

    let mut shapes = Vec::new();
    let mut at_zones = Vec::new();

    // Walk forward, judging each candle only on what came before it.
    for end in 2..=bars.len() {
        let history: Vec<&Bar> = bars[..end].iter().collect();

        let Some(normal) = normal_candle(&history, 14) else {
            continue;
        };

        if let Some(shape) = pattern::ending_at(&history, normal, named) {
            shapes.push((name_of(shape), history[end - 1].datetime.clone()));
        }

        // The same call the bot makes. Not a second reading of the rules.
        if let Ok(signal) = nsc_strategy::look(&history, bands, normal, named, rules) {
            at_zones.push(signal);
        }
    }

    Ok(Found {
        symbol: pair.symbol.clone(),
        interval,
        digits: pair.digits,
        last,
        shapes,
        at_zones,
        bands: bands.to_vec(),
    })
}

/// What the code calls a shape.
fn name_of(pattern: Pattern) -> String {
    format!("{pattern:?}")
        .split(' ')
        .next()
        .unwrap_or("?")
        .to_string()
}
