//! Every price that comes down the line, against the bands he drew.
//!
//! **Almost all of them are nowhere near anything.** Prices arrive about once a
//! second and barely move, so this says nothing on the overwhelming majority
//! of them — which is the point.

use std::collections::HashMap;

use anyhow::Result;
use chrono::Utc;
use nsc_core::levels::{News, Thickness};
use rust_decimal::Decimal;
use tokio_tungstenite::tungstenite::Message;

use super::{Watching, pulse, say};

pub async fn heard(
    client: &reqwest::Client,
    watching: &mut HashMap<String, Watching>,
    thickness: Thickness,
    heard: &Message,
    pulse: &mut pulse::Pulse,
) -> Result<()> {
    let Ok(said) = serde_json::from_str::<serde_json::Value>(&heard.to_string()) else {
        return Ok(());
    };

    if said["event"] != "price" {
        return Ok(());
    }

    let (Some(symbol), Some(price)) = (said["symbol"].as_str(), price_in(&said)) else {
        return Ok(());
    };

    let Some(seen) = watching.get_mut(symbol) else {
        return Ok(());
    };

    let reach = seen.pair.reach(thickness);

    for (band, near) in seen.watch.arrive(price) {
        println!("{symbol} reached {}", band.price);

        // **A card that will not send is not the price line breaking.**
        // Letting it out of here dropped a perfectly good socket and told him
        // the feed was down. It has already tried three times by now.
        match say::alert(client, &seen.pair, &band, near, News::Fresh, price, reach).await {
            Ok(()) => pulse.spoke(Utc::now()),
            Err(trouble) => eprintln!("Could not send that alert: {trouble:#}"),
        }
    }

    Ok(())
}

/// The price out of a message, as a Decimal — never through a float.
fn price_in(said: &serde_json::Value) -> Option<Decimal> {
    said["price"]
        .as_str()
        .and_then(|text| text.parse().ok())
        .or_else(|| {
            said["price"]
                .as_f64()
                .and_then(|number| Decimal::try_from(number).ok())
        })
}
