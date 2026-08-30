//! Rung 3 — a shape he trades, at a level he drew.
//!
//! **Only ever on a finished candle.** Rung 2 has a "so far" card that reads a
//! candle still running, and it says so on its face. A *signal* must never do
//! that: a shape halfway through a candle is not a shape, and one that
//! un-forms before the close would have been a message about something that
//! never happened.

use std::path::{Path, PathBuf};

use chrono::Utc;
use nsc_core::candle::{Bar, normal_candle};
use nsc_core::levels::Band;
use nsc_data::source::Interval;
use nsc_strategy::{Signal, look, reasons};
use nsc_ta::pattern;

use super::look::Closes;
use super::said::{Kind, Said};
use crate::places::{OWNER, PREVIEW};
use crate::retry::keep_trying;
use crate::watch::{Watching, pulse};
use crate::{card, telegram};

/// How many candles a "normal" one is averaged over. Fourteen is the usual.
const NORMAL_OVER: usize = 14;

/// How many candles **the run** shows — the widest of the three pictures.
///
/// **His ask, 30 August 2026:** *"so that I see the direction the price has
/// been coming from... if it's coming from down to up, or if it's doing a
/// curve, or if it's going from up to down."*
///
/// Four hundred is about two and a half weeks on the 1-hour and nine on the
/// 4-hour. **The whole swing, not the last few days.**
const RUN: usize = 400;

/// How many candles **the close-up** shows, the one carrying the red ring.
///
/// **His number, 29 August 2026:** *"a hundred candles, so I can see what
/// played out, how it played out."*
const CONTEXT: usize = 100;

impl Closes {
    /// Looks for a signal on the candle that just finished, and sends it.
    ///
    /// `bars` are newest first, as the feed hands them over.
    ///
    /// **Nothing here can end the run.** A card that will not send is not the
    /// price line breaking. It says what went wrong and tries again next look.
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn setup(
        &mut self,
        client: &reqwest::Client,
        seen: &Watching,
        live: &[Band],
        bars: &[Bar],
        finished: &Bar,
        interval: Interval,
        patterns: &pattern::Rules,
        rules: &nsc_strategy::Rules,
        pulse: &mut pulse::Pulse,
    ) {
        // **Oldest first, and cut at the candle being judged.** `look` reads
        // backwards from the end, so anything after it in the list would be
        // price the market had not printed when this candle closed.
        let mut history: Vec<&Bar> = bars.iter().rev().collect();

        let Some(at) = history
            .iter()
            .position(|bar| bar.datetime == finished.datetime)
        else {
            return;
        };

        history.truncate(at + 1);

        // **Normal AT THAT MOMENT, not today.** Judged against today's, a
        // shape from last week is measured against a market that had not
        // happened yet when it printed.
        let Some(normal) = normal_candle(&history, NORMAL_OVER) else {
            return;
        };

        let Some(signal) = look(&history, live, normal, patterns, rules) else {
            return;
        };

        // Once per candle per zone, like everything else here. A shape does
        // not become a second shape because the next look found it again.
        let key = Said {
            symbol: seen.pair.symbol.clone(),
            interval,
            kind: Kind::Setup,

            band: signal.standing.band().price.to_string(),
        };

        if self.already_said(&key, &finished.datetime) {
            return;
        }

        let written = card::as_written(interval);

        println!(
            "SETUP — {}",
            reasons::sentence(&signal, &seen.pair.symbol, written, seen.pair.digits)
        );

        match send(client, &signal, seen, live, &history, written).await {
            Ok(()) => {
                pulse.spoke(Utc::now());
                self.told.insert(key, finished.datetime.clone());
            }

            // Deliberately not remembered, so the next look tries again.
            Err(trouble) => eprintln!("Could not send that setup: {trouble:#}"),
        }
    }
}

/// Draws the card and sends it.
///
/// **Chrome runs off the price loop.** Drawing is a blocking wait of two to
/// ten seconds; left in the async task it holds a Tokio worker for all of it,
/// which stops everything on the one-core box this is meant to be hosted on.
async fn send(
    client: &reqwest::Client,
    signal: &Signal,
    seen: &Watching,
    live: &[Band],
    history: &[&Bar],
    written: &str,
) -> anyhow::Result<()> {
    let words = reasons::sentence(signal, &seen.pair.symbol, written, seen.pair.digits);
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
    //     the run       400 candles, no ring    where price CAME FROM
    //     the close-up  100 candles, red ring   where the shape PRINTED
    //     the card      the shape itself        WHAT it was
    //
    // Sent together as one message. Any one of them alone leaves an obvious
    // question unanswered.
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

    let owner = OWNER.to_string();

    // **Widest first, then in.** The run, the close-up, then the shape — you
    // step toward it rather than away from it.
    let pictures = [three[0].as_path(), three[1].as_path(), three[2].as_path()];

    keep_trying(3, || telegram::send_to(client, &owner, &pictures, &words)).await?;

    Ok(())
}

/// Reads the two settings rung 3 needs, once at startup.
pub fn settings(
    strategy: &str,
    patterns: &str,
) -> anyhow::Result<(nsc_strategy::Rules, pattern::Rules)> {
    let rules = nsc_strategy::load(Path::new(strategy))?;
    let named = pattern::load(Path::new(patterns))?;

    Ok((rules, named))
}
