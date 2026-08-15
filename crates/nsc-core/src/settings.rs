//! Everything a trader would tune, in one place.
//!
//! All of it is written into the code right now, which is honest for one pair
//! and would be a mess for two. **Step 2 replaces this file with reading
//! `config/`** — the project rule is that anything tunable lives in a TOML
//! file, and this is the one place that breaks it on purpose, temporarily.

/// The pair. Twelve Data writes them with a slash.
pub const SYMBOL: &str = "XAU/USD";

/// The timeframe, in Twelve Data's spelling.
pub const INTERVAL: &str = "1h";

/// How long that timeframe is. Used to work out whether a candle has finished,
/// which is the single most important question this program asks.
pub const INTERVAL_MINUTES: i64 = 60;

/// How many candles to fetch and draw. Five days of hours — enough to see the
/// shape, few enough that each candle is still a candle rather than a hair.
pub const HISTORY: usize = 120;

/// How many decimal places this instrument is quoted to.
///
/// Gold is two. Twelve Data sends five — `4385.59525` — which is the raw feed
/// rather than anything a chart shows. Printing all five is what makes a
/// message read like a debug dump.
pub const DIGITS: u32 = 2;

/// `1h` reads badly in a message and `1day` reads worse.
///
/// Taken from the interval rather than written into each card, so changing the
/// interval cannot leave the wording behind saying something untrue.
pub fn timeframe_name(interval: &str) -> &str {
    match interval {
        "1h" => "1 hour",
        "4h" => "4 hour",
        "1day" => "daily",
        "1week" => "weekly",
        other => other,
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
