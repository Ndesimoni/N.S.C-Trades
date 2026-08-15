//! The one line that goes under the picture.
//!
//! It is not the message — the card is. This is what shows in the
//! **notification banner** before Telegram is even opened, so it carries the
//! four things worth knowing at a glance and nothing else.
//!
//! Telegram gives text no font size, no colour and no layout. Anything that
//! has to look good goes on the card instead.

use rust_decimal::Decimal;

use crate::candle::Bar;
use crate::settings::{DIGITS, INTERVAL, SYMBOL, timeframe_name};

/// Builds the caption.
///
/// Nothing here can fail, so it does not pretend it might.
pub fn build(bar: &Bar) -> String {
    let rose = bar.change() >= Decimal::ZERO;

    format!(
        "<b>{SYMBOL}</b>  ·  {}  ·  <b>{}</b>  {} {}%",
        timeframe_name(INTERVAL),
        bar.close.round_dp(DIGITS),
        if rose { "▲" } else { "▼" },
        bar.change_percent().abs(),
    )
}
