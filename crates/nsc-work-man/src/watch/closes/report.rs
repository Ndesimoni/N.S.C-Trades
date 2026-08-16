//! Saying what a candle did, one zone at a time.

use chrono::Utc;
use nsc_core::candle::Bar;
use nsc_core::levels::{Band, Thickness, action, what_it_did};

use super::look::Closes;
use super::said::{Kind, Said};
use crate::watch::{Watching, pulse, say};

impl Closes {
    /// Says what one candle did at every zone price is at.
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
        interval: &'static str,
        forming: bool,
        pulse: &mut pulse::Pulse,
    ) {
        let kind = if forming { Kind::SoFar } else { Kind::Closed };

        for band in live {
            let key = Said {
                symbol: seen.pair.symbol.clone(),
                interval,
                kind,
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

            let was = action(band, bar, thickness.kiss_depth);
            let what = if forming { "so far" } else { "closed" };

            println!(
                "{} {interval} candle {} — {what} {was:?} at {}",
                seen.pair.symbol, bar.datetime, band.price
            );

            match say::closed(client, &seen.pair, band, bar, did, was, interval, forming).await {
                Ok(()) => {
                    pulse.spoke(Utc::now());
                    self.told.insert(key, bar.datetime.clone());
                }

                // Deliberately not remembered, so the next look tries it again.
                Err(trouble) => eprintln!("Could not send that one: {trouble:#}"),
            }
        }
    }
}
