//! Every price that comes down the line, against the bands he drew.
//!
//! **Almost all of them are nowhere near anything.** Prices arrive several
//! times a second and barely move, so this says nothing on the overwhelming
//! majority of them — which is the point.
//!
//! **The price is the middle of the spread**, worked out in
//! `nsc-data::sources::ibkr::ticks` from the last bid and the last ask. It has
//! to be, because the candles come back as mid prices: measured against a bid,
//! a level would look reached when the candle says it never was.

use std::collections::HashMap;

use anyhow::Result;
use chrono::Utc;
use nsc_core::levels::{News, Thickness};
use nsc_data::source::Price;

use super::{Watching, pulse, say};

pub async fn heard(
    client: &reqwest::Client,
    watching: &mut HashMap<String, Watching>,
    thickness: Thickness,
    heard: Price,
    pulse: &mut pulse::Pulse,
    settled: bool,
) -> Result<()> {
    // A pair he stopped watching while the line was open. Its subscription
    // outlives the decision by a moment.
    let Some(seen) = watching.get_mut(&heard.symbol) else {
        return Ok(());
    };

    let reach = seen.pair.reach(thickness);

    for (band, near) in seen.watch.arrive(heard.mid) {
        println!("{} reached {}", heard.symbol, band.price);

        // **Watched, remembered, not spoken about.** `arrive` has already run,
        // so where price is stays true through the opening hours — the report
        // at the end of them says where it actually stands.
        if !settled {
            continue;
        }

        // **A card that will not send is not the price line breaking.**
        // Letting it out of here dropped a perfectly good line and told him
        // the feed was down. It has already tried three times by now.
        match say::alert(
            client,
            &seen.pair,
            &band,
            near,
            News::Fresh,
            heard.mid,
            reach,
        )
        .await
        {
            Ok(()) => pulse.spoke(Utc::now()),
            Err(trouble) => eprintln!("Could not send that alert: {trouble:#}"),
        }
    }

    Ok(())
}
