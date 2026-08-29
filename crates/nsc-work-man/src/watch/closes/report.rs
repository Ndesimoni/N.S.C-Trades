//! Saying what a candle did, one zone at a time.

use chrono::Utc;
use nsc_core::candle::Bar;
use nsc_core::levels::{Band, Thickness, action, what_it_did};
use nsc_data::source::Interval;

use super::look::Closes;
use super::said::{Kind, Said};
use crate::card;
use crate::watch::{Watching, pulse, say};

impl Closes {
    /// Says what one candle did at every zone price is at.
    ///
    /// **Only finished candles reach here.** The mid-candle "so far" card was
    /// taken out on 27 August 2026 — two messages per zone visit and never
    /// three, and with it went the only place in this project that read a
    /// candle still running.
    ///
    /// **Each zone is decided on its own.** They used to be remembered
    /// together, which cost twice: a second zone coming live mid-hour never
    /// got that hour's candle at all, and one card failing to send made every
    /// other zone on that candle repeat on the next look.
    ///
    /// **Nothing here can end the run.** A card that will not send is not the
    /// price line breaking, and treating it as one would drop a perfectly good
    /// socket and tell him the feed is down. It says what went wrong, does not
    /// remember that zone, and tries again on the next look.
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn say(
        &mut self,
        client: &reqwest::Client,
        seen: &Watching,
        live: &[Band],
        bar: &Bar,
        thickness: Thickness,
        interval: Interval,
        pulse: &mut pulse::Pulse,
    ) -> bool {
        let mut all_sent = true;

        // The card templates were written against the feed's own spelling.
        // One place turns the type back into it — see `card/spelling.rs`.
        let written = card::as_written(interval);

        for band in live {
            let key = Said {
                symbol: seen.pair.symbol.clone(),
                interval,
                kind: Kind::Closed,
                band: band.price.to_string(),
            };

            if self.already_said(&key, &bar.datetime) {
                continue;
            }

            let did = what_it_did(band, bar);

            // **Not remembered when there is nothing to say.** A forming candle
            // is still moving — nothing worth saying at twenty minutes in may
            // be worth saying at forty, and the stamp is the same one. Working
            // it out again is arithmetic on candles already fetched.
            if !did.worth_saying() {
                continue;
            }

            // **A candle that settled INSIDE the zone says nothing he does not
            // already know.** He gets an alert when price arrives; a candle
            // closing there as well is the same news twice.
            //
            // The rejection survives: a wick into the zone that closed back
            // out finishes above or below the band, so it still sends.
            if thickness.only_breaks && !did.left_the_band() {
                continue;
            }

            let was = action(band, bar, thickness.kiss_depth);

            println!(
                "{} {written} candle {} — closed {was:?} at {}",
                seen.pair.symbol, bar.datetime, band.price
            );

            match say::closed(client, &seen.pair, band, bar, did, was, written).await {
                Ok(()) => {
                    pulse.spoke(Utc::now());
                    self.told.insert(key, bar.datetime.clone());
                }

                // Deliberately not remembered, so the next look tries it
                // again — and `when_next` brings that look forward, because
                // waiting for the following candle is four hours on the
                // 4-hour.
                Err(trouble) => {
                    eprintln!("Could not send that one: {trouble:#}");
                    all_sent = false;
                }
            }
        }

        all_sent
    }
}
