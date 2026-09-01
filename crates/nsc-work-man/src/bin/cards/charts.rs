//! **The two chart pictures, on every pair he watches.**
//!
//! Asked for on 1 September 2026, right after the candle counts were cut:
//! *"send me a test for all pairs, let me see it."* One message per pair, each
//! carrying the wide run and the close-up.
//!
//! ## It uses the real thing at every step
//!
//! His own levels out of `config/pairs`, sized off real candles the way the
//! watcher sizes them, drawn on real candles from IBKR. Nothing here is a
//! stand-in, because a preview drawn from made-up numbers can only tell you
//! the drawing code runs — not whether the picture is any good.
//!
//! **The candle counts come from the watcher**, `RUN` and `CONTEXT`. That is
//! the whole point: this is the picture used to judge those two numbers, so it
//! must not be able to hold its own copy of them. It did, once, and it drew a
//! hundred candles of a bot that had been cut to forty-five.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use nsc_core::candle::{Bar, normal_candle};
use nsc_core::levels::{self, Band, Pair, Thickness};
use nsc_data::source::Interval;
use nsc_data::sources::ibkr::IbkrConnection;
use nsc_strategy::look;
use nsc_ta::pattern;
use nsc_work_man::places::{OWNER, PAIRS, PATTERNS, PREVIEW, STRATEGY};
use nsc_work_man::watch::{CONTEXT, RUN, size_bands};
use nsc_work_man::{card, telegram};

use super::candles;

/// How many candles a "normal" one is averaged over. Fourteen is the usual.
const NORMAL_OVER: usize = 14;

/// Every pair in `config/pairs`, in the order the folder lists them.
pub async fn every_pair(client: &reqwest::Client, ibkr: &IbkrConnection) -> Result<()> {
    let thickness = levels::load_thickness(Path::new(nsc_work_man::places::THICKNESS))?;
    let rules = nsc_strategy::load(Path::new(STRATEGY))?;
    let patterns = pattern::load(Path::new(PATTERNS))?;

    let mut files: Vec<PathBuf> = std::fs::read_dir(PAIRS)
        .with_context(|| format!("could not read {PAIRS}"))?
        .filter_map(|entry| entry.ok().map(|one| one.path()))
        .filter(|path| path.extension().is_some_and(|kind| kind == "toml"))
        .collect();

    files.sort();

    for file in files {
        // **One pair failing does not stop the rest.** A pair IBKR will not
        // give candles for is worth saying out loud and walking past; it is
        // not a reason to send him nothing at all.
        if let Err(trouble) = one_pair(client, ibkr, &file, thickness, &patterns, &rules).await {
            eprintln!("{}: {trouble:#}", file.display());
        }
    }

    Ok(())
}

async fn one_pair(
    client: &reqwest::Client,
    ibkr: &IbkrConnection,
    file: &Path,
    thickness: Thickness,
    patterns: &pattern::Rules,
    rules: &nsc_strategy::Rules,
) -> Result<()> {
    let pair = levels::load_pair(file)?;

    // Sized exactly as the watcher sizes them — same function, same requests.
    let bands = size_bands(ibkr, &pair, thickness).await?;
    let hourly = candles(ibkr, &pair.symbol, Interval::H1).await?;

    let history: Vec<&Bar> = hourly.iter().collect();
    let last =
        |many: usize| -> Vec<&Bar> { history[history.len().saturating_sub(many)..].to_vec() };

    // **A ring only if a shape actually finished on the newest candle.** A
    // ring drawn round nothing teaches him to stop trusting the ring.
    let ring = normal_candle(&history, NORMAL_OVER)
        .and_then(|normal| look(&history, &bands, normal, patterns, rules))
        .map(|signal| signal.shape.candles());

    let stem = pair.symbol.replace('/', "");
    let run_out = PathBuf::from(PREVIEW).join(format!("run-{stem}.png"));
    let near_out = PathBuf::from(PREVIEW).join(format!("close-up-{stem}.png"));

    let pictures = [
        card::render(
            "chart.html",
            &last(RUN),
            &bands,
            &pair.symbol,
            "1h",
            pair.digits,
            &run_out,
        )?,
        card::render_ringed(
            "chart.html",
            &last(CONTEXT),
            &bands,
            &pair.symbol,
            "1h",
            pair.digits,
            ring,
            &near_out,
        )?,
    ];

    let words = words_for(&pair, &bands, ring);
    println!("  {words}");

    let paths: Vec<&Path> = pictures.iter().map(PathBuf::as_path).collect();
    telegram::send_to(client, &OWNER.to_string(), &paths, &words).await?;

    Ok(())
}

/// What the message says. **It never calls this a signal**, because it is not
/// one — it is a picture of the pair as it stands.
fn words_for(pair: &Pair, bands: &[Band], ring: Option<usize>) -> String {
    let zones = match bands.len() {
        1 => "1 zone".to_string(),
        many => format!("{many} zones"),
    };

    let shape = match ring {
        Some(_) => " · a shape finished on the last candle",
        None => "",
    };

    format!(
        "{} 1h — {RUN} candles, then {CONTEXT} · {zones}{shape}",
        pair.symbol
    )
}
