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
            band: signal.band.price.to_string(),
        };

        if self.already_said(&key, &finished.datetime) {
            return;
        }

        let written = card::as_written(interval);

        println!(
            "SETUP — {}",
            reasons::sentence(&signal, &seen.pair.symbol, written, seen.pair.digits)
        );

        match send(client, &signal, seen, &history, written).await {
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
    history: &[&Bar],
    written: &str,
) -> anyhow::Result<()> {
    let words = reasons::sentence(signal, &seen.pair.symbol, written, seen.pair.digits);
    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();

    // The two candles the shape is made of, oldest first.
    let shown: Vec<Bar> = history
        .iter()
        .rev()
        .take(2)
        .rev()
        .map(|&bar| bar.clone())
        .collect();

    let signal = signal.clone();
    let pair = seen.pair.clone();
    let timeframe = written.to_string();
    let out = PathBuf::from(PREVIEW).join("setup.png");

    let picture: PathBuf = tokio::task::spawn_blocking(move || {
        let borrowed: Vec<&Bar> = shown.iter().collect();
        card::setup(&signal, &pair, &borrowed, &timeframe, &stamp, &out)
    })
    .await??;

    let owner = OWNER.to_string();
    let pictures = [picture.as_path()];

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
