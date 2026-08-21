//! The middle of the spread, and which notices are worth passing on.

use ibapi::contracts::tick_types::TickType;
use ibapi::messages::Notice;
use rust_decimal::Decimal;

use crate::source::Heard;

use super::listening::from_notice;
use super::spread::Spread;

fn about(text: &str) -> Decimal {
    text.parse().expect("a price")
}

fn notice(code: i32) -> Notice {
    Notice {
        code,
        message: "something".into(),
        error_time: None,
    }
}

/// **One side of the market is not a price.**
///
/// A bid on its own says what somebody will pay, not what the market is. Sent
/// as a price it would sit half a spread below every band edge — and on gold
/// that is most of the edge.
#[test]
fn nothing_comes_back_until_both_sides_have_arrived() {
    let mut spread = Spread::default();

    assert_eq!(spread.took(TickType::Bid, 1.16412), None);
    assert_eq!(spread.took(TickType::Ask, 1.16414), Some(about("1.16413")));
}

/// The middle is halfway, and it is exact.
#[test]
fn the_middle_is_halfway_between_the_two() {
    let mut spread = Spread::default();

    spread.took(TickType::Bid, 4120.10);

    assert_eq!(spread.took(TickType::Ask, 4120.40), Some(about("4120.25")));
}

/// **A market that has not moved says nothing.**
///
/// Prices arrive several times a second and nearly all of them describe the
/// same market as the one before. Repeating them would put the whole loop to
/// work on news that is not news.
#[test]
fn a_middle_that_has_not_moved_is_not_reported() {
    let mut spread = Spread::default();

    spread.took(TickType::Bid, 1.16412);
    assert!(spread.took(TickType::Ask, 1.16414).is_some());

    // The same ask again, and a bid that leaves the middle where it was.
    assert_eq!(spread.took(TickType::Ask, 1.16414), None);
    assert_eq!(spread.took(TickType::Bid, 1.16412), None);
}

/// **One side moving IS the middle moving**, and it has to be reported.
///
/// The two sides never arrive together, so waiting for both to move would
/// throw away half of every real move. Widening by a tick on the bid alone
/// walks the middle half a tick down, and that is a price.
#[test]
fn one_side_moving_moves_the_middle() {
    let mut spread = Spread::default();

    spread.took(TickType::Bid, 1.16412);
    assert_eq!(spread.took(TickType::Ask, 1.16414), Some(about("1.16413")));

    // Only the bid moved. The middle went with it.
    assert_eq!(spread.took(TickType::Bid, 1.16410), Some(about("1.16412")));

    // And now the ask follows, putting the middle back where it started.
    assert_eq!(spread.took(TickType::Ask, 1.16416), Some(about("1.16413")));
}

/// **Coming back to a price it has already been is still news.**
///
/// Only the price immediately before is compared against. Remembering every
/// price ever seen would silence a market ticking between two numbers — which
/// is exactly what price does while it sits on one of his levels.
#[test]
fn a_price_it_has_been_before_is_still_reported() {
    let mut spread = Spread::default();

    spread.took(TickType::Bid, 4120.10);
    assert_eq!(spread.took(TickType::Ask, 4120.40), Some(about("4120.25")));

    assert_eq!(spread.took(TickType::Bid, 4120.20), Some(about("4120.30")));
    assert_eq!(spread.took(TickType::Bid, 4120.10), Some(about("4120.25")));
}

/// **IBKR sends -1 to mean "no price".**
///
/// Taken as a number it is a price impossibly far below every level, and the
/// first band it passed would fire.
#[test]
fn minus_one_is_not_a_price() {
    let mut spread = Spread::default();

    assert_eq!(spread.took(TickType::Bid, -1.0), None);
    assert_eq!(spread.took(TickType::Ask, 1.16414), None);

    // And it did not quietly keep the -1 as the bid either.
    assert_eq!(spread.took(TickType::Bid, 1.16412), Some(about("1.16413")));
}

/// **A price that is not a number is not a price.**
///
/// NaN compares false against everything, so a check on the sign alone would
/// wave one through — and the middle of a real bid and a NaN ask is a NaN,
/// which then compares false against every band edge and silences the pair.
#[test]
fn a_price_that_is_not_a_number_is_refused() {
    let mut spread = Spread::default();

    assert_eq!(spread.took(TickType::Bid, f64::NAN), None);
    assert_eq!(spread.took(TickType::Ask, f64::INFINITY), None);

    // Neither was kept, so a real pair of prices still works.
    spread.took(TickType::Bid, 1.16412);
    assert_eq!(spread.took(TickType::Ask, 1.16414), Some(about("1.16413")));
}

/// Sizes, last-traded prices and the rest are not the two sides of the market.
#[test]
fn only_the_bid_and_the_ask_build_the_middle() {
    let mut spread = Spread::default();

    spread.took(TickType::Bid, 1.16412);

    assert_eq!(spread.took(TickType::Last, 1.16413), None);
    assert_eq!(spread.took(TickType::BidSize, 3_000_000.0), None);
}

/// **The data farms saying hello are not a refusal.**
///
/// 2104 arrives on every single connection. Passed on, it would report a
/// perfectly healthy feed as refused every time the bot started.
#[test]
fn the_farm_chatter_is_ignored() {
    for code in [2104, 2106, 2158, 1101, 1102] {
        assert!(
            from_notice("EUR/USD", notice(code)).is_none(),
            "{code} should be ignored",
        );
    }
}

/// A missing market data subscription must reach the watcher.
///
/// It is the failure that otherwise looks exactly like a quiet market: the
/// line stays open, nothing errors, and no price ever arrives.
#[test]
fn a_refusal_is_passed_on() {
    let heard = from_notice("XAU/USD", notice(354));

    assert!(matches!(heard, Some(Heard::Refused { .. })));
}

/// TWS losing its own connection to IB is the line breaking, not a refusal.
/// One is worth opening again; the other never is.
#[test]
fn losing_the_data_farm_is_the_line_breaking() {
    let heard = from_notice("EUR/USD", notice(1100));

    assert!(matches!(heard, Some(Heard::Broke { .. })));
}
