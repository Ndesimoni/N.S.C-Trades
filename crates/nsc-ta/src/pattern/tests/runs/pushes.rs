//! Runs of two that make — or just miss — his own pattern.
//!
//! **Every one of these printed.** Pulled out of live IBKR data on 21 August
//! 2026 by sweeping all five pairs in `config/pairs` across every timeframe
//! from 30 minutes up. Each carries the normal candle that was true at the
//! moment it printed, because a 2025 run judged against a 2026 yardstick is
//! measured against a market that had not happened yet.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use super::making::bar;

fn d(text: &str) -> Decimal {
    text.parse().expect("a price")
}

/// **The pair off his own screenshot.** Gold's daily, 19 and 20 August 2026 —
/// the two candles he circled and asked about. A push of 1.9x a normal day
/// with 87% body, then a tail of 65 points under a body of five.
pub(in crate::pattern::tests) fn his_gold() -> (Vec<Bar>, Decimal) {
    (
        vec![
            bar(
                "2026-08-19 00:00:00",
                "4344.53",
                "4524.36",
                "4324.71",
                "4517.78",
            ),
            bar(
                "2026-08-20 00:00:00",
                "4520.67",
                "4541.06",
                "4450.71",
                "4515.78",
            ),
        ],
        d("104.27462"),
    )
}

/// The same thing upside down — gold's daily on 2 June 2026. A belt-hold down,
/// then a rally that was sold.
pub(in crate::pattern::tests) fn bear_gold() -> (Vec<Bar>, Decimal) {
    (
        vec![
            bar(
                "2026-06-01 00:00:00",
                "4553.5",
                "4553.5",
                "4448.0",
                "4485.1",
            ),
            bar(
                "2026-06-02 00:00:00",
                "4480.95",
                "4541.65",
                "4463.03",
                "4490.05",
            ),
        ],
        d("105.14538"),
    )
}

/// **A pin whose body is exactly nothing** — open and close to the tick.
///
/// The tail-to-body rule cannot score this one, because there is nothing to
/// divide by. It passes on the nose cap alone: with no body and almost no
/// nose, what is left has to be tail.
pub(in crate::pattern::tests) fn no_body_pin() -> (Vec<Bar>, Decimal) {
    (
        vec![
            bar(
                "2026-08-14 01:00:00",
                "1.15385",
                "1.15385",
                "1.1532",
                "1.1534",
            ),
            bar(
                "2026-08-14 01:30:00",
                "1.1534",
                "1.1538",
                "1.15335",
                "1.1534",
            ),
        ],
        d("0.00032"),
    )
}

/// **A pin whose body is exactly one third** — the biggest his rule allows.
///
/// `config/candles.toml` calls this candle `plain`, because its `body.small`
/// is 0.33 and this body works out at 0.3333. The pattern has its own cap at
/// 0.3334 and takes it. That is the whole reason the setting was not shared.
pub(in crate::pattern::tests) fn exactly_a_third() -> (Vec<Bar>, Decimal) {
    (
        vec![
            bar(
                "2026-08-14 01:00:00",
                "1.34975",
                "1.3498",
                "1.349",
                "1.34915",
            ),
            bar(
                "2026-08-14 01:30:00",
                "1.34915",
                "1.34955",
                "1.34895",
                "1.34895",
            ),
        ],
        d("0.00044"),
    )
}

/// **The tail points the SAME way as the push.** A push down, then a long tail
/// down. Euro daily, 5 January 2026.
pub(in crate::pattern::tests) fn tail_with_the_push() -> (Vec<Bar>, Decimal) {
    (
        vec![
            bar(
                "2026-01-02 00:00:00",
                "1.1751",
                "1.1765",
                "1.17135",
                "1.172",
            ),
            bar(
                "2026-01-05 00:00:00",
                "1.17245",
                "1.1729",
                "1.1659",
                "1.1722",
            ),
        ],
        d("0.00460"),
    )
}

/// **Both shapes right, and the push went nowhere.** Euro daily, 19 August
/// 2025: a long body met by a long upper wick, but the push reached only 0.73
/// of a normal candle.
pub(in crate::pattern::tests) fn push_too_small() -> (Vec<Bar>, Decimal) {
    (
        vec![
            bar(
                "2025-08-18 00:00:00",
                "1.1712",
                "1.17165",
                "1.1656",
                "1.1661",
            ),
            bar(
                "2025-08-19 00:00:00",
                "1.1662",
                "1.1693",
                "1.1639",
                "1.16465",
            ),
        ],
        d("0.00824"),
    )
}

/// **A pin that covered more ground than the push it answers.** Gold's
/// 4-hour, 29 July 2026: a push of 37 points met by a pin of 109 — nearly
/// three times the size.
///
/// Everything else about it is right. The push is 80% body, it reached 1.6
/// normal candles, and the pin has a long tail the correct way with no nose.
/// It is refused on size alone.
pub(in crate::pattern::tests) fn pin_bigger_than_push() -> (Vec<Bar>, Decimal) {
    (
        vec![
            bar(
                "2026-07-29 12:00:00",
                "4032.83",
                "4033.46",
                "3996.13",
                "4009.76",
            ),
            bar(
                "2026-07-29 16:00:00",
                "4009.76",
                "4116.48",
                "4007.71",
                "4044.76",
            ),
        ],
        d("23.60692"),
    )
}
