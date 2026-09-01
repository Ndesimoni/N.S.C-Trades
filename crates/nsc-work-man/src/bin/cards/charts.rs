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
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::IbkrConnection;
use nsc_strategy::{look, reasons};
use nsc_ta::pattern;
use nsc_work_man::card::{CONTEXT, RUN};
use nsc_work_man::places::{OWNER, PAIRS, PATTERNS, PREVIEW, STRATEGY};
use nsc_work_man::watch::size_bands;
use nsc_work_man::{card, telegram};

use nsc_work_man::retry::keep_trying;

/// How many candles a "normal" one is averaged over. Fourteen is the usual.
const NORMAL_OVER: usize = 14;

/// How many hourly candles to ask IBKR for.
///
/// **`main.rs` asks for 60 and that is right for what it does** — it only
/// needs enough to work out a normal candle, which takes fourteen. This needs
/// enough to DRAW: 200 for the run picture, and then room behind that to walk
/// back looking for the last shape.
///
/// Asking for 60 is what made every pair report "nothing printed" on the first
/// try: the search floor is the run length, so with 60 candles in hand there
/// was not one window wide enough to judge, and the loop never ran once.
const FETCH: usize = 1500;

/// How far back to look for a shape, in candles.
///
/// **Most pairs have nothing on the newest candle**, which is the whole point
/// of the bot — it is quiet nearly all the time. To show him what a signal
/// looks like on each pair, this walks backwards until it finds one.
///
/// It stops at the first hit, so it is instant when something printed recently
/// and only walks the whole way when a pair has been quiet for weeks.
const LOOK_BACK: usize = 2000;

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

    let mut hourly = keep_trying(3, || ibkr.candles(&pair.symbol, Interval::H1, FETCH))
        .await
        .with_context(|| format!("could not get hourly candles for {}", pair.symbol))?;

    // They arrive newest first. A chart is read the other way.
    hourly.reverse();

    let all: Vec<&Bar> = hourly.iter().collect();

    // **The most recent candle that finished a shape at one of his levels.**
    // Nearly every pair is quiet on the newest candle — that is the bot
    // working — so to show him what a signal looks like on each one, this
    // walks back to the last real one.
    let found = newest(&all, &bands, patterns, rules);

    // **The chart ends where the shape did**, so the ring lands on the last
    // candle drawn. With nothing found it ends at today and wears no ring.
    let end = found.as_ref().map_or(all.len(), |(at, _)| *at);
    let signal = found.map(|(_, one)| one);
    let history = &all[..end];

    let last =
        |many: usize| -> Vec<&Bar> { history[history.len().saturating_sub(many)..].to_vec() };

    let ring = signal.as_ref().map(|one| one.shape.candles());

    let stem = pair.symbol.replace('/', "");
    let run_out = PathBuf::from(PREVIEW).join(format!("run-{stem}.png"));
    let near_out = PathBuf::from(PREVIEW).join(format!("close-up-{stem}.png"));

    let mut pictures = vec![
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

    // **The third picture only exists when there is a shape to draw.** This is
    // the same bundle the watcher sends — the run, the close-up with the ring,
    // and the card naming what printed.
    if let Some(found) = &signal {
        pictures.push(card::setup(
            found,
            &pair,
            &last(found.shape.candles()),
            "1h",
            &history[history.len() - 1].datetime,
            &PathBuf::from(PREVIEW).join(format!("setup-{stem}.png")),
        )?);
    }

    let words = match &signal {
        Some(found) => format!(
            "{}\n{}",
            reasons::sentence(found, &pair.symbol, "1h", pair.digits),
            history[history.len() - 1].datetime
        ),
        None => words_for(&pair, &bands),
    };
    println!("  {words}");

    let paths: Vec<&Path> = pictures.iter().map(PathBuf::as_path).collect();
    telegram::send_to(client, &OWNER.to_string(), &paths, &words).await?;

    Ok(())
}

/// What the message says when **nothing printed**. It never calls this a
/// signal, because it is not one — it is a picture of the pair as it stands.
///
/// When a shape did print, `reasons::sentence` writes the words instead, which
/// is the same sentence the watcher would have sent.
fn words_for(pair: &Pair, bands: &[Band]) -> String {
    let zones = match bands.len() {
        1 => "1 zone".to_string(),
        many => format!("{many} zones"),
    };

    format!(
        "{} 1h — {RUN} candles, then {CONTEXT} · {zones} · nothing printed",
        pair.symbol
    )
}

/// The most recent candle that completed a shape at one of his levels, and the
/// signal it made. Newest first, stopping at the first hit.
///
/// **Nothing here can see forwards.** Each try hands `look` the candles up to
/// and including the one being judged and no more, which is the same slice the
/// watcher gives it live.
fn newest(
    all: &[&Bar],
    bands: &[Band],
    patterns: &pattern::Rules,
    rules: &nsc_strategy::Rules,
) -> Option<(usize, nsc_strategy::Signal)> {
    let floor = RUN.max(NORMAL_OVER + 1);
    let oldest = all.len().saturating_sub(LOOK_BACK).max(floor);

    for end in (oldest..=all.len()).rev() {
        let upto = &all[..end];

        // **Skip a window that cannot be measured; do not abandon the search.**
        // It was `?`, which returned out of the whole function — so one
        // unmeasurable window would have reported the pair as quiet rather
        // than carrying on to the next.
        let Some(normal) = normal_candle(upto, NORMAL_OVER) else {
            continue;
        };

        if let Some(signal) = look(upto, bands, normal, patterns, rules) {
            return Some((end, signal));
        }
    }

    None
}
