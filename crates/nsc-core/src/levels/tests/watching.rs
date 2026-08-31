//! Firing once per touch, not once per price. What a CLOSE at a level is
//! worth is `breaking.rs`.

use rust_decimal::Decimal;

use super::super::{Band, Nearness, Timeframe, Watch, nearness};
use super::support::{aussie, d, gold, share};

/// This band's own reach, for tests calling `nearness` directly.
fn reach() -> Decimal {
    gold().thickness() * share()
}

fn watching() -> Watch {
    Watch::over(vec![gold()], share())
}

// ── A touch, not a crossing ──

// Price at 4132.60 has touched the top in every way that matters.
#[test]
fn a_price_just_outside_still_counts_as_arriving() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    let arrived = watch.arrive(d("4132.60"));

    assert_eq!(arrived.len(), 1, "a touch is an arrival");
    assert_eq!(arrived[0].1, Nearness::Approaching, "and it says which");
}

#[test]
fn price_inside_the_band_says_so() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(watch.arrive(d("4100"))[0].1, Nearness::Inside);
}

// The slack is a twentieth of the band, NOT time to react. The band already
// gives him that — its top is about three hours of gold movement.
#[test]
fn the_slack_is_a_twentieth_and_no_more() {
    let band = gold();

    let grazing = nearness(&band, d("4132.60"), reach());
    assert_eq!(grazing, Nearness::Approaching);
    assert_eq!(nearness(&band, d("4132.50"), reach()), Nearness::Inside);

    // A twentieth of this band is $3.86, so a dollar out is still a touch. It
    // was not when this setting was ten cents, and gold is why it changed.
    let dollar_out = nearness(&band, d("4133.60"), reach());
    assert_eq!(dollar_out, Nearness::Approaching);

    assert_eq!(nearness(&band, d("4140"), reach()), Nearness::Away);
}

#[test]
fn how_close_counts_is_a_setting() {
    let band = gold();

    // Nothing counts but the band itself.
    assert_eq!(nearness(&band, d("4132.60"), d("0")), Nearness::Away);

    // A pip of slack reaches it.
    assert_eq!(
        nearness(&band, d("4132.60"), d("0.10")),
        Nearness::Approaching
    );

    // Ten pips reaches further.
    assert_eq!(nearness(&band, d("4133.50"), d("0.10")), Nearness::Away);
    assert_eq!(
        nearness(&band, d("4133.50"), d("1.00")),
        Nearness::Approaching
    );
}

// ── Once per touch ──

// The reason the type exists. Prices come about once a second and barely move.
//
// Once price is INSIDE, going further in says nothing — it is already as deep
// as it gets.
#[test]
fn sitting_inside_fires_nothing_more() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(watch.arrive(d("4132.50")).len(), 1, "it went in");

    for price in ["4130", "4120", "4100", "4060"] {
        assert!(
            watch.arrive(d(price)).is_empty(),
            "{price} was already inside"
        );
    }
}

// THE ONE HE ASKED FOR. Price drifts up to the zone, then walks into it.
//
// Coming near used to mark the band as reached, so walking IN was not a change
// and said nothing at all — he heard "coming up on your zone" and then waited
// for a candle, which on the hourly is up to an hour. Entering is the thing he
// actually wanted to know.
#[test]
fn coming_near_and_then_going_in_is_one_message() {
    let mut watch = watching();
    watch.arrive(d("4300"));

    let near = watch.arrive(d("4132.60"));
    assert_eq!(near.len(), 1);
    assert_eq!(near[0].1, Nearness::Approaching, "it says price is close");

    // **It used to send a second card here** — "and now it is in". He asked
    // for one on 31 August: *"price comes up to the level, approaching — the
    // one card."*
    assert!(
        watch.arrive(d("4132.50")).is_empty(),
        "going in is not a second card"
    );
}

// And never more than those two. Drifting back out to the edge and in again is
// one visit, however many times it happens.
#[test]
fn going_in_and_out_of_the_edge_is_still_one_visit() {
    let mut watch = watching();
    watch.arrive(d("4300"));
    watch.arrive(d("4132.60"));
    watch.arrive(d("4132.50"));

    let mut fired = 0;
    for price in ["4132.60", "4132.50", "4132.65", "4130", "4132.62"] {
        fired += watch.arrive(d(price)).len();
    }

    assert_eq!(fired, 0, "in and out of the same edge, five times");
}

#[test]
fn leaving_and_coming_back_says_nothing() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(watch.arrive(d("4100")).len(), 1, "the first visit speaks");

    watch.arrive(d("4300"));

    // **This used to fire again, and that was the complaint.** A level speaks
    // once; after that only a candle CLOSING somewhere new is news.
    assert!(
        watch.arrive(d("4100")).is_empty(),
        "coming back is not news — 31 August"
    );
}

// Price hovering at the edge crosses it again and again, and describes one
// moment where nothing happened.
#[test]
fn hovering_at_the_edge_does_not_fire_over_and_over() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(
        watch.arrive(d("4132.60")).len(),
        1,
        "the first touch counts"
    );

    // All still OUTSIDE the band and inside its reach — the same depth it
    // already reported, five times over.
    let mut fired = 0;
    for price in ["4132.65", "4132.62", "4132.67", "4132.58", "4132.61"] {
        fired += watch.arrive(d(price)).len();
    }

    assert_eq!(fired, 0, "the same moment, told five times");
}

// Leaving is a REAL distance — a pip out then a pip back must not be a second
// alert for one visit.
#[test]
fn once_it_has_spoken_it_stays_quiet() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(watch.arrive(d("4132.60")).len(), 1, "the one card");

    assert!(watch.arrive(d("4143")).is_empty(), "hovering is not news");
    assert!(watch.arrive(d("4132.60")).is_empty(), "nor is coming back");

    // **Properly gone, and it still says nothing.** Leaving used to re-arm
    // the card; on 31 August he asked for silence until a candle CLOSES
    // somewhere new. See `breaking.rs`.
    watch.arrive(d("4145"));
    assert!(
        watch.arrive(d("4132.60")).is_empty(),
        "a real trip away is still not a new card"
    );
}

// ── Several bands ──

// Only the one price reached; the other is 1,100 away.
#[test]
fn only_the_band_price_arrived_at_fires() {
    let daily = Band::around(Timeframe::Daily, d("2984"), d("70.36"), d("0.46"));
    let mut watch = Watch::over(vec![gold(), daily], share());

    watch.arrive(d("3500"));

    let arrived = watch.arrive(d("4100"));
    assert_eq!(arrived.len(), 1);
    assert_eq!(arrived[0].0.timeframe, Timeframe::Weekly);
}

#[test]
fn a_price_near_nothing_fires_nothing() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert!(watch.arrive(d("4280")).is_empty());
}

// ── The overlap he found live, 31 August 2026 ──

#[test]
fn a_wobble_inside_the_approach_does_not_fire_again() {
    let mut watch = Watch::over(vec![aussie()], share());

    watch.arrive(d("0.71800"));

    assert_eq!(
        watch.arrive(d("0.71390")).len(),
        1,
        "arriving at the band is one alert"
    );

    // 0.71360 — still approaching, and under where the OLD reset line sat.
    assert!(
        watch.arrive(d("0.71360")).is_empty(),
        "drifting within the approach is not a new arrival"
    );

    assert!(
        watch.arrive(d("0.71390")).is_empty(),
        "and coming back from it is not either — this is the bug he saw"
    );
}
