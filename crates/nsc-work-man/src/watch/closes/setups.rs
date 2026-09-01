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
/// **Cut from four hundred on 1 September 2026**, the same day and for the
/// same reason as the close-up: *"reduce the candles in the first chart too,
/// so we can see it clear."*
///
/// At four hundred the bodies were on their floor — 1.5 units, about 3px — so
/// the picture was a texture rather than candles. Two hundred gives 2.0
/// units, about 3.8px, and you can tell one candle from the next.
///
/// **It still shows the whole move**, which is its only job: on the AUD/USD
/// hourly it is 8 days, and that carried the drop, the base, the push up, the
/// top and the pull back into the level with room to spare.
///
/// **Two hundred since 1 September 2026**, his number: *"for the empty run
/// let it be 200 candles, not 150."* The empty run is this one — the wide
/// chart with no ring on it. About 11 days on the 1-hour, 33 on the 4-hour,
/// 9 months on the daily.
pub const RUN: usize = 200;

/// How many candles **the close-up** shows, the one carrying the red ring.
///
/// **Cut from a hundred on 1 September 2026:** *"even when I zoom into the
/// picture I still do not see the setup clearly."*
///
/// A hundred was his own number a few days earlier, and it was the right
/// answer to the question he was asking then — *"so I can see what played out,
/// how it played out."* It was the wrong number for SEEING it.
///
/// **The candles are drawn to fit, so the count IS the size.** The plot is 728
/// units wide and 82% of that is drawn, which is 597 for however many candles
/// there are:
///
/// ```text
///     100 candles    body  3.6 units   wick 0.6    ~7px and ~1px on the phone
///      45 candles    body  9.0 units   wick 2.0   ~17px and ~4px
/// ```
///
/// **The wick was the half that broke it.** At a hundred candles it came out
/// under a pixel, and a pin bar IS its wick — so the one shape that most needs
/// to be seen was the one being rounded away. Zooming a picture cannot put
/// back a line that was never drawn.
///
/// Forty-five is about 2 days on the 1-hour, 7.5 on the 4-hour and 9 weeks on
/// the daily. **The run picture still carries the history**; this one only has
/// to show the shape and what walked into the level.
pub const CONTEXT: usize = 45;

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
    //     the run      200 candles, no ring    where price CAME FROM
    //     the close-up   45 candles, red ring   where the shape PRINTED
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
