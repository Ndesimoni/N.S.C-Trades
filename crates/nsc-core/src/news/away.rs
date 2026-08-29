//! How long until it prints, in words.

use chrono::{DateTime, Utc};

use super::Event;

/// Minutes in an hour and in a day, named so the arithmetic below reads.
const HOUR: i64 = 60;
const DAY: i64 = 24 * HOUR;

/// "in 45m", "in 3h 20m", "in 2d 4h" — or that it has already gone.
///
/// **Worked out here, not on the card.** Every card in this project is told
/// what it says out loud rather than reading a clock of its own, which is what
/// lets the wording be tested without a browser.
///
/// **The units shrink as it gets closer**, because that is what he needs. Two
/// days out, the hours do not matter. Forty minutes out, they are the only
/// thing that does — and "in 0h" for something forty minutes away is the kind
/// of rounding that reads as a card that failed to fill in.
pub fn away_words(event: &Event, now: DateTime<Utc>) -> String {
    let minutes = (event.at - now).num_minutes();

    if minutes < 0 {
        return "passed".to_string();
    }

    if minutes == 0 {
        return "now".to_string();
    }

    if minutes < HOUR {
        return format!("in {minutes}m");
    }

    if minutes < DAY {
        let hours = minutes / HOUR;
        let left = minutes % HOUR;

        return if left == 0 {
            format!("in {hours}h")
        } else {
            format!("in {hours}h {left}m")
        };
    }

    let days = minutes / DAY;
    let hours = (minutes % DAY) / HOUR;

    if hours == 0 {
        format!("in {days}d")
    } else {
        format!("in {days}d {hours}h")
    }
}
