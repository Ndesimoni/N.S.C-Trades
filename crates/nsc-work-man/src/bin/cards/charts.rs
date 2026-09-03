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
//! **The candle counts come from `card`**, `RUN` and `CONTEXT` — this is the
//! picture used to judge those two numbers, so it must not hold its own copy.
//! It did once, and drew a hundred candles of a bot cut to forty-five.

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
    // **Everything up to the shape, handed over whole.** `card::render` keeps
    // the last RUN of them and `render_ringed` the last CONTEXT, so slicing
    // here as well would be a second place to get it wrong — which is exactly
    // how the review chart came to draw three hundred.
    let history = &all[..end];

    let ring = signal.as_ref().map(|one| one.shape.candles());

    let stem = pair.symbol.replace('/', "");
    let run_out = PathBuf::from(PREVIEW).join(format!("run-{stem}.png"));
    let near_out = PathBuf::from(PREVIEW).join(format!("close-up-{stem}.png"));

    // Same shape as the live path — see `watch/closes/drawing.rs`.
    let charts = [
        card::render_marked(
            "chart.html",
            history,
            &bands,
            &pair.symbol,
            "1h",
            pair.digits,
            // **Only framed when there is a setup.** A pair with nothing on it
            // is two charts, not two thirds of a signal.
            match ring {
                Some(_) => card::Mark::part(1),
                None => card::Mark::plain(),
            },
            &run_out,
        )?,
        card::render_marked(
            "chart.html",
            history,
            &bands,
            &pair.symbol,
            "1h",
            pair.digits,
            match ring {
                Some(many) => card::Mark::ringed(2, many),
                None => card::Mark::plain(),
            },
            &near_out,
        )?,
    ];

    // **The card only exists when there is a shape to draw.** Same bundle the
    // watcher sends: the run, the close-up with the ring, and the card naming
    // what printed — the card last, with the buttons on it.
    let mut card_out = None;

    if let Some(found) = &signal {
        card_out = Some(card::setup(
            found,
            &pair,
            &history[history.len().saturating_sub(found.shape.candles())..],
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

    // **One container** — see `watch/closes/drawing.rs` for the trade. With no
    // shape there are only the two charts and nothing to tap.
    let owner = OWNER.to_string();

    let mut group: Vec<&Path> = charts.iter().map(PathBuf::as_path).collect();

    if let Some(card) = &card_out {
        group.push(card.as_path());
    }

    telegram::send_to(client, &owner, &group, &words).await?;

    // ── AND THE TWO BUTTONS, IF THERE IS A RECORD TO HANG THEM ON ──
    //
    // **The whole point of doing it here.** The buttons carry a signal's row
    // id, so they cannot exist without a row — which means the only way to see
    // them before the bot next finds a live setup is to record one.
    //
    // The row is honest: a real shape, at a real level he drew, on a candle
    // that really closed. What it is not is NEW — `candle_opened_at` says when
    // it printed, which may be days ago.
    if let Some(found) = &signal {
        super::asking::ask_him(client, &pair, found, history, &words).await;
    }

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

        if let Ok(signal) = look(upto, bands, normal, patterns, rules) {
            return Some((end, signal));
        }
    }

    None
}
