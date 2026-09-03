//! Rung 3 — a shape he trades, at a level he drew.
//!
//! **Only ever on a finished candle.** Rung 2 has a "so far" card that reads a
//! candle still running, and it says so on its face. A *signal* must never do
//! that: a shape halfway through a candle is not a shape, and one that
//! un-forms before the close would have been a message about something that
//! never happened.

use chrono::Utc;
use nsc_core::candle::{Bar, normal_candle};
use nsc_core::levels::Band;
use nsc_data::source::Interval;
use nsc_strategy::{look, reasons};
use nsc_ta::pattern;
use std::path::Path;

use super::drawing::draw;
use super::look::Closes;
use super::recording;
use super::said::{Kind, Said};
use crate::card;
use crate::retry::keep_trying;
use crate::telegram;
use crate::watch::{Watching, pulse};

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

        // ── THE DECISION, AND IT IS WRITTEN DOWN EITHER WAY ──
        //
        // `CLAUDE.md`: *"Rejected setups get saved, not thrown away. Save
        // which layer rejected them."* A quiet week and a broken bot look
        // identical from outside, and so do "nothing printed" and "forty
        // printed and none was near a level" — which are completely different
        // problems.
        //
        // A candle with no shape at all is NOT written. That is nearly every
        // candle, and `Refused::worth_keeping` is where the line sits.
        let signal = match look(&history, live, normal, patterns, rules) {
            Ok(signal) => signal,

            Err(why) => {
                recording::keep_refusal(
                    self.record.as_ref(),
                    &self.rules_version,
                    recording::Missed {
                        pair: &seen.pair,
                        interval,
                        bar: finished,
                        why: &why,
                        normal,
                    },
                )
                .await;

                return;
            }
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

        let sentence = reasons::sentence(&signal, &seen.pair.symbol, written, seen.pair.digits);

        // **All three as one group.** The buttons follow in their own message,
        // because a group of photos cannot carry them — see `drawing.rs`.
        let pictures = match draw(&signal, seen, live, &history, written).await {
            Ok(pictures) => Some(pictures),

            Err(trouble) => {
                eprintln!("Could not draw that setup: {trouble:#}");
                None
            }
        };

        let mut sent_at = None;

        if let Some(pictures) = &pictures {
            let owner = crate::places::OWNER.to_string();
            let group = [
                pictures[0].as_path(),
                pictures[1].as_path(),
                pictures[2].as_path(),
            ];

            match keep_trying(3, || telegram::send_to(client, &owner, &group, &sentence)).await {
                Ok(()) => {
                    pulse.spoke(Utc::now());
                    self.told.insert(key, finished.datetime.clone());
                    sent_at = Some(Utc::now());
                }

                Err(trouble) => eprintln!("Could not send that setup: {trouble:#}"),
            }
        }

        // **Recorded whether or not Telegram took it.** The bot saw this, and
        // a signal missing from the record because a message failed would make
        // the history disagree with what the rules actually did. `sent_at`
        // being null is how the two are told apart.
        //
        // **The shape's FIRST candle**, since a march spans three and the
        // record wants to know where the shape starts, not only where it
        // finished.
        let spans_from = history[history.len().saturating_sub(signal.shape.candles())];

        recording::keep_signal(
            client,
            self.record.as_ref(),
            &self.rules_version,
            recording::Made {
                pair: &seen.pair,
                interval,
                bar: finished,
                spans_from,
                signal: &signal,
                normal,
                sentence: &sentence,
                sent_at,
            },
        )
        .await;
    }
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
