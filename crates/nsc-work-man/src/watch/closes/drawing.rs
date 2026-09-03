//! **Drawing a setup** — the three pictures, stacked into one.
//!
//! Split out of `setups.rs` on 2 September 2026, when recording the decision
//! pushed that file past the limits. It is a clean seam: deciding whether
//! there is a setup and drawing one are two jobs, and only one of them needs
//! Chrome.

use std::path::PathBuf;

use chrono::Utc;
use nsc_core::candle::Bar;
use nsc_core::levels::Band;
use nsc_strategy::Signal;

use crate::card;
use crate::card::{CONTEXT, RUN};
use crate::places::PREVIEW;
use crate::watch::Watching;

/// Draws the card and sends it.
///
/// **Chrome runs off the price loop.** Drawing is a blocking wait of two to
/// ten seconds; left in the async task it holds a Tokio worker for all of it,
/// which stops everything on the one-core box this is meant to be hosted on.
pub(super) async fn draw(
    signal: &Signal,
    seen: &Watching,
    live: &[Band],
    history: &[&Bar],
    written: &str,
) -> anyhow::Result<[PathBuf; 3]> {
    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();

    // **The candles the shape is made of, oldest first — and it asks the shape
    // how many.** Marching is three; taking two would draw two thirds of a
    // pattern and label it whole.
    let shown: Vec<Bar> = history
        .iter()
        .rev()
        .take(signal.shape.candles())
        .rev()
        .map(|&bar| bar.clone())
        .collect();

    // **Three pictures, and each answers a different question.**
    //
    //     the run      200 candles, no ring    where price CAME FROM
    //     the close-up   45 candles, red ring   where the shape PRINTED
    //     the card      the shape itself        WHAT it was
    //
    // Any one of them alone leaves an obvious question unanswered.
    //
    // **THREE SEPARATE MESSAGES**, and that is the point of them.
    //
    // A GROUP OF PHOTOS GETS CROPPED side by side in the chat, which on a
    // chart this tall hides most of it and cannot be opened one at a time.
    // Stacked into one image they were readable but could only expand
    // together, and looked cramped besides.
    //
    // Sent one after another, each is full width and each opens on its own
    // tap. His words, 3 September 2026: *"I want three different cards, and I
    // want to click an individual card and have it expand."*
    //
    // The last one carries the buttons — Telegram allows them on a single
    // photo and never on a group.
    let take_last = |many: usize| -> Vec<Bar> {
        history
            .iter()
            .rev()
            .take(many)
            .rev()
            .map(|&bar| bar.clone())
            .collect()
    };

    let run = take_last(RUN);
    let context = take_last(CONTEXT);

    let signal = signal.clone();
    let pair = seen.pair.clone();
    let bands = live.to_vec();
    let timeframe = written.to_string();
    let ring = signal.shape.candles();

    let run_out = PathBuf::from(PREVIEW).join("signal-run.png");
    let wide_out = PathBuf::from(PREVIEW).join("signal-chart.png");
    let card_out = PathBuf::from(PREVIEW).join("setup.png");

    // **Both drawn in ONE hop off the price loop.** Chrome is a blocking wait
    // of two to ten seconds each; two separate `spawn_blocking` calls would
    // hold two of the pool's threads instead of one.
    let three: [PathBuf; 3] = tokio::task::spawn_blocking(move || {
        let whole: Vec<&Bar> = run.iter().collect();
        let far: Vec<&Bar> = context.iter().collect();
        let near: Vec<&Bar> = shown.iter().collect();

        // **No ring on the run.** It is there to show the shape of the move,
        // and a ring at the far right of four hundred candles would be a dot
        // pointing at nothing readable.
        let run = card::render(
            "chart.html",
            &whole,
            &bands,
            &pair.symbol,
            &timeframe,
            pair.digits,
            &run_out,
        )?;

        let wide = card::render_ringed(
            "chart.html",
            &far,
            &bands,
            &pair.symbol,
            &timeframe,
            pair.digits,
            Some(ring),
            &wide_out,
        )?;

        let close_up = card::setup(&signal, &pair, &near, &timeframe, &stamp, &card_out)?;

        Ok::<_, card::CardError>([run, wide, close_up])
    })
    .await??;

    // **Widest first, then in.** The run, the close-up, then the shape — he
    // steps toward it rather than away from it.
    Ok(three)
}
