//! Holding the price line open, and everything that can end it.

use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::levels::Thickness;
use nsc_core::when::{self, Allowed, Rules};
use nsc_data::source::Heard;
use nsc_data::sources::ibkr::IbkrConnection;
use tokio::sync::watch as tell_of;

use super::closed::Closed;
use super::refusals::Refusals;
use crate::watch::run::snapshot;
use crate::watch::standing::Snapshot;
use crate::watch::{Kit, Watching, prices};

/// Holds the line open and watches every price that comes down it.
///
/// **Returns when the line closes, when the session goes quiet, or when he
/// sends a level.** The caller decides what happens next; this does not.
///
/// **One subscription per pair, folded into one channel.** Twelve Data carried
/// every symbol on a single socket. IBKR gives one connection per contract, so
/// `nsc-data` folds them back together — and this loop is unchanged by that,
/// which is the whole point of the fold.
pub async fn listen(
    client: &reqwest::Client,
    ibkr: &IbkrConnection,
    watching: &mut HashMap<String, Watching>,
    thickness: Thickness,
    calendar: &Rules,
    kit: &mut Kit,
    tell: &tell_of::Sender<Snapshot>,
) -> Result<Closed> {
    let symbols: Vec<String> = watching.keys().cloned().collect();

    let mut line = ibkr
        .prices(&symbols)
        .await
        .context("the price line would not open")?;

    // **How many are actually being listened to, not how many were asked
    // for.** A pair IBKR would not subscribe to at all never gets a line, so
    // counting the asking would leave this waiting for a refusal that can
    // never arrive.
    let mut refusals = Refusals::watching(line.watching());

    loop {
        tokio::select! {
            heard = line.next() => {
                let Some(heard) = heard else {
                    // Every pair's line has ended and the last sender is gone.
                    anyhow::bail!("every price line closed");
                };

                match heard {
                    Heard::Price(price) => {
                        // **The price is recorded first, and the order
                        // matters.**
                        //
                        // The greeting reports which zones price is RESTING IN,
                        // and nothing is resting anywhere until a price has
                        // been fed in. On the very first price of a session the
                        // greeting used to run first, find nothing, send
                        // nothing — and mark the session greeted. The report of
                        // where price already stood never came at all.
                        prices::heard(watching, price);

                        kit.awake
                            .greet(client, watching, thickness, calendar, &mut kit.pulse)
                            .await?;
                    }

                    // **IBKR refusing a pair is silent otherwise.** It sends
                    // one notice down a line that stays open and then never
                    // sends a price — which is indistinguishable from a pair
                    // nothing is happening to.
                    Heard::Refused { symbol, why } => {
                        if refusals.and_that_is_everything(&symbol, &why) {
                            anyhow::bail!(
                                "IBKR refused every pair: {}",
                                refusals.what_they_said()
                            );
                        }
                    }

                    // **A line that ended is trouble, not a clean finish.** It
                    // leaves by the error path so the five-minute rule decides
                    // whether he hears about it — quiet about hiccups, loud
                    // about outages.
                    Heard::Broke { symbol, why } => {
                        anyhow::bail!("the price line for {symbol} ended: {why}");
                    }
                }
            }

            _ = kit.closes.next_check() => {
                kit.closes.tick();

                // The heartbeat is checked here as well as in `run`, because a
                // busy line means this loop is where the time is spent.
                kit.pulse.maybe(client, watching, calendar).await?;

                // Gone quiet — the weekend, or Monday. Hand back and let `run`
                // put the line away rather than draining one nobody is reading.
                if when::allowed(Utc::now(), calendar) == Allowed::Silence {
                    println!("The session has closed. Standing down.");
                    return Ok(Closed::Line);
                }

                // Checked by the clock on the files, so the normal answer —
                // nothing happened — costs one look at a folder.
                if kit.files.changed() {
                    println!("The levels changed. Reading them again.");
                    return Ok(Closed::LevelsChanged);
                }

                // Held through the opening hours, same as the price alerts.
                // The first candle report after the window covers what
                // happened during it.
                if when::settled(Utc::now(), calendar) {
                    kit.closes
                        .look(client, ibkr, watching, thickness, calendar, &mut kit.pulse)
                        .await?;
                }

                // Where price last was has moved on. Anything asking /status
                // should see today, not where it stood when the line opened.
                let _ = tell.send(snapshot(watching, calendar));
            }
        }
    }
}
