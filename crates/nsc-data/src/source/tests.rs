//! What a timeframe is worth, and that no two of them are the same thing.

use super::Interval;

/// Every interval there is. Written out so that adding one to the enum and
/// forgetting it here is a failing test rather than a silent gap.
const ALL: [Interval; 7] = [
    Interval::Min5,
    Interval::Min15,
    Interval::Min30,
    Interval::H1,
    Interval::H4,
    Interval::Day,
    Interval::Week,
];

/// **The four-hour is four hours.**
///
/// This is not restating the table. A constant sixty sat where this number
/// goes once, and it called every 4-hour candle finished three hours early —
/// which does not error, it just reports a candle the market has not printed.
#[test]
fn the_four_hour_is_four_hours() {
    assert_eq!(Interval::H4.minutes(), 4 * 60);
    assert_eq!(Interval::H1.minutes(), 60);
}

/// No two intervals may claim the same length.
///
/// A copy-paste in the match arms is invisible by eye — `Min15 => 15` under
/// `Min30 => 15` reads perfectly well. It would make the 30-minute candle
/// report as finished twice as often as it exists.
#[test]
fn no_two_are_the_same_length() {
    for (at, one) in ALL.iter().enumerate() {
        for other in &ALL[at + 1..] {
            assert_ne!(
                one.minutes(),
                other.minutes(),
                "{one:?} and {other:?} claim the same length",
            );
        }
    }
}

/// They get longer in the order they are written.
///
/// The enum's order is relied on by eye everywhere it is matched on. One
/// arm out of place is the sort of thing nobody reads twice.
#[test]
fn they_run_shortest_to_longest() {
    for pair in ALL.windows(2) {
        assert!(
            pair[0].minutes() < pair[1].minutes(),
            "{:?} is not shorter than {:?}",
            pair[0],
            pair[1],
        );
    }
}

/// Each one has its own name, and it is a name a person would say.
#[test]
fn each_has_its_own_spoken_name() {
    let mut said: Vec<&str> = ALL.iter().map(|one| one.spoken()).collect();
    said.sort_unstable();

    let how_many = said.len();
    said.dedup();

    assert_eq!(said.len(), how_many, "two intervals share a name");
    assert!(said.iter().all(|name| !name.is_empty()));
}
