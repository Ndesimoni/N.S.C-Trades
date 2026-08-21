//! What a boundary measurement is allowed to conclude.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use super::{agreed_on, hour_of, line_up, voted, weekday_of};

fn bar(datetime: &str, open: &str) -> Bar {
    let price: Decimal = open.parse().expect("a price");

    Bar {
        datetime: datetime.to_string(),
        open: price,
        high: price,
        low: price,
        close: price,
    }
}

/// The hour that shares the day's opening price is where the day began.
#[test]
fn the_hour_with_the_same_open_is_the_boundary() {
    let daily = [bar("2026-08-19 00:00:00", "4482.15")];
    let hourly = [
        bar("2026-08-18 20:00:00", "4479.00"),
        bar("2026-08-18 21:00:00", "4482.15"),
        bar("2026-08-18 22:00:00", "4483.60"),
    ];

    let lined = line_up(&daily, &hourly);

    assert_eq!(lined[0].started, vec!["2026-08-18 21:00:00"]);
    assert_eq!(agreed_on(&lined, hour_of), Some("21:00".to_string()));
}

/// **Candles that disagree prove nothing, and must not be averaged into an
/// answer.** Two boundaries in one sample means the sample is wrong, or
/// the feed is — either way it is not a measurement.
#[test]
fn candles_that_disagree_settle_nothing() {
    let daily = [
        bar("2026-08-19 00:00:00", "1.10"),
        bar("2026-08-20 00:00:00", "1.20"),
    ];
    let hourly = [
        bar("2026-08-18 21:00:00", "1.10"),
        bar("2026-08-19 22:00:00", "1.20"),
    ];

    assert_eq!(agreed_on(&line_up(&daily, &hourly), hour_of), None);
}

/// **A price two hours share settles nothing either.** A quiet market does
/// that, and taking the first would be a guess wearing a measurement's
/// clothes.
#[test]
fn a_price_two_hours_share_is_not_an_answer() {
    let daily = [bar("2026-08-19 00:00:00", "1.10")];
    let hourly = [
        bar("2026-08-18 21:00:00", "1.10"),
        bar("2026-08-18 22:00:00", "1.10"),
    ];

    let lined = line_up(&daily, &hourly);

    assert_eq!(lined[0].started.len(), 2);
    assert_eq!(voted(&lined), 0);
}

/// When nothing matches, how far off the nearest was is the useful answer.
#[test]
fn it_says_how_far_the_nearest_was() {
    let daily = [bar("2026-08-19 00:00:00", "4482.15")];
    let hourly = [bar("2026-08-18 21:00:00", "4482.17")];

    let lined = line_up(&daily, &hourly);

    assert!(lined[0].started.is_empty());
    assert_eq!(
        lined[0].nearest.as_ref().unwrap().1,
        "0.02".parse().unwrap()
    );
}

/// **Agreeing is not the same as being enough**, and the caller has to ask
/// both. On EUR/USD two candles out of six voted, agreed with each other, and
/// the first version printed EVERY CANDLE AGREES.
#[test]
fn agreeing_and_being_enough_are_two_questions() {
    let daily = [
        bar("2026-08-18 00:00:00", "1.10"),
        bar("2026-08-19 00:00:00", "1.20"),
        bar("2026-08-20 00:00:00", "1.30"),
    ];
    let hourly = [
        bar("2026-08-17 21:15:00", "1.10"),
        // 1.20 is shared, so that candle cannot vote.
        bar("2026-08-18 21:15:00", "1.20"),
        bar("2026-08-18 22:15:00", "1.20"),
        // and nothing opened at 1.30 at all.
    ];

    let lined = line_up(&daily, &hourly);

    assert_eq!(agreed_on(&lined, hour_of), Some("21:15".to_string()));
    assert_eq!(
        voted(&lined),
        1,
        "one vote out of three is not a measurement"
    );
}

/// For the week, the weekday is the answer and the clock time is noise.
#[test]
fn the_week_is_measured_in_days_not_hours() {
    assert_eq!(
        weekday_of("2026-08-17 00:00:00"),
        Some("Monday".to_string())
    );
    assert_eq!(hour_of("2026-08-17 00:00:00"), Some("00:00".to_string()));
}
