//! What a pair's name tells you about it.
//!
//! **Worked out, not checked.** Enough to start a file with, and the file it
//! writes says so — so he can correct anything that behaves differently.

/// `GBPUSD` is how he types it. `GBP/USD` is how the feed wants it.
pub fn with_slash(name: &str) -> String {
    if name.contains('/') || name.len() != 6 {
        return name.to_string();
    }

    format!("{}/{}", &name[..3], &name[3..])
}

/// How many decimals a pair is quoted to, from its name.
///
/// **Worked out, not checked.** Enough to start a file with, and the file says
/// so.
pub fn digits_for(name: &str) -> u32 {
    let upper = name.to_uppercase();

    if upper.starts_with("XAU") || upper.starts_with("XAG") || upper.starts_with("XCU") {
        2
    } else if upper.contains("JPY") {
        3
    } else {
        5
    }
}

/// Metals and oil shut for an hour at 17:00 New York every weekday. Spot forex
/// runs straight through.
pub(super) fn nightly_break(name: &str) -> i64 {
    let upper = name.to_uppercase();

    if upper.starts_with('X') || upper.contains("OIL") {
        60
    } else {
        0
    }
}

/// What one unit of this instrument is.
///
/// Gold is priced per troy ounce. A currency pair is priced in the second
/// currency, one unit of the first.
pub fn unit_for(symbol: &str) -> String {
    match symbol {
        "XAU/USD" | "XAG/USD" => "USD / troy oz".into(),
        other => other.replace('/', " / "),
    }
}
