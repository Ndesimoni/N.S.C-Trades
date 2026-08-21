//! What a card calls a timeframe.

use nsc_data::source::Interval;

/// The interval, spelled the way the card templates expect it.
///
/// **The cards were written against the old feed's spelling** — `1h`, `4h`,
/// `1week` — and `nsc_core::candle::timeframe_name` turns those into the words
/// on the card. Keeping that spelling here means the timeframe became a type
/// without a single card changing what it says.
///
/// One place does it. The strings used to be written out at the call sites,
/// and a typo in one of those does not fail to compile — it fails on one
/// timeframe, at runtime, while everything else carries on.
pub fn as_written(interval: Interval) -> &'static str {
    match interval {
        Interval::Min5 => "5min",
        Interval::Min15 => "15min",
        Interval::Min30 => "30min",
        Interval::H1 => "1h",
        Interval::H4 => "4h",
        Interval::Day => "1day",
        Interval::Week => "1week",
    }
}
