//! What a timeframe is called, and how long it lasts.
//!
//! **Worked out from the feed's own spelling**, so changing which timeframe a
//! card is drawn on cannot leave the wording — or the finish time — behind
//! saying something untrue.

/// `1h` reads badly in a message and `1day` reads worse.
pub fn timeframe_name(interval: &str) -> &str {
    match interval {
        "1h" => "1 hour",
        "4h" => "4 hour",
        "1day" => "daily",
        "1week" => "weekly",
        other => other,
    }
}

/// How many minutes that timeframe lasts.
///
/// **`None` for anything not recognised, and the caller must not guess.** This
/// is the number that decides whether a candle has finished, and a wrong guess
/// reads a candle before it exists — the one mistake in this project that
/// makes results look better rather than broken.
///
/// A day and a week are deliberately absent. Their length is not a fixed run
/// of minutes: the trading day ends at 17:00 New York, and both change length
/// twice a year when the clocks move.
pub fn minutes_for(interval: &str) -> Option<i64> {
    match interval {
        "1min" => Some(1),
        "5min" => Some(5),
        "15min" => Some(15),
        "30min" => Some(30),
        "1h" => Some(60),
        "2h" => Some(120),
        "4h" => Some(240),
        _ => None,
    }
}
