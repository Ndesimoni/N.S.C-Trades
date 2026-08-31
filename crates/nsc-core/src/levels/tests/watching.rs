//! Firing once per touch, not once per price.

use rust_decimal::Decimal;

use super::super::{Band, Nearness, Timeframe, Watch, nearness};
use super::support::d;

/// A twentieth of whatever band it is asked about.
///
/// **A share, not a price, since 31 August 2026.** It was ten cents here — one
/// gold pip — which is 0.13% of this band and 22% of an AUD/USD daily one. The
/// same setting meaning two different things is what broke it.
fn share() -> Decimal {
    d("0.05")
}

/// The gold band he drew at 4094 — 4055.43 to 4132.57, about 77.15 thick.
///
/// A touch now reaches **4136.43**, a twentieth of the band past the edge, and
/// the band goes quiet again only past **4144.15** — a tenth of the band clear
/// of where approaching ends.
///
/// **Both grew when the setting became a share, and gold is why it had to.**
/// At ten cents its approach zone was 0.13% of the band, so price went from
/// Away straight to Inside with almost nothing in between — gold had no
/// approach warning at all.
fn gold() -> Band {
    Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"))
}

/// This band's own reach, for the tests that call `nearness` directly.
fn reach() -> Decimal {
    gold().thickness() * share()
}

fn watching() -> Watch {
    Watch::over(vec![gold()], share())
}

// ── A touch, not a crossing ──

// The band's top is 4132.57. Price at 4132.60 has touched it in every way that
// matters, and staying quiet over three cents would be silly.
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
// gives him that — its top is about three hours of gold movement from the line
// he drew.
#[test]
fn the_slack_is_a_twentieth_and_no_more() {
    let band = gold();

    assert_eq!(
        nearness(&band, d("4132.60"), reach()),
        Nearness::Approaching
    );
    assert_eq!(nearness(&band, d("4132.50"), reach()), Nearness::Inside);

    // A twentieth of this band is $3.86, so a dollar out is still a touch —
    // it was not when this was ten cents, and gold is the pair that needed
    // the change.
    assert_eq!(
        nearness(&band, d("4133.60"), reach()),
        Nearness::Approaching
    );

    // Past 4136.43 it is properly away.
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
fn coming_near_and_then_going_in_are_two_different_messages() {
    let mut watch = watching();
    watch.arrive(d("4300"));

    let near = watch.arrive(d("4132.60"));
    assert_eq!(near.len(), 1);
    assert_eq!(near[0].1, Nearness::Approaching, "first, that it is close");

    let inside = watch.arrive(d("4132.50"));
    assert_eq!(inside.len(), 1);
    assert_eq!(inside[0].1, Nearness::Inside, "then, that it is in");
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
fn leaving_and_coming_back_fires_again() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    watch.arrive(d("4100"));
    watch.arrive(d("4300"));

    assert_eq!(watch.arrive(d("4100")).len(), 1, "a second visit");
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

// Leaving is a REAL distance. A pip back out must not reset the band, or the
// next pip back in is a second alert for one visit.
#[test]
fn leaving_takes_more_than_it_took_to_arrive() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    watch.arrive(d("4132.60"));

    // Well outside a touch, still nowhere near clear of the band.
    assert!(watch.arrive(d("4143")).is_empty(), "not gone yet");
    assert!(
        watch.arrive(d("4132.60")).is_empty(),
        "so coming back is not new"
    );

    // Past a tenth of the band beyond a touch — 4144.15 — it is properly gone.
    watch.arrive(d("4145"));
    assert_eq!(
        watch.arrive(d("4132.60")).len(),
        1,
        "now it is a fresh touch"
    );
}

// ── Several bands ──

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

/// His AUD/USD daily level at 0.71500, on a 49.3-pip daily candle.
///
/// The band comes out **22.7 pips** thick — 0.713865 to 0.716135.
fn aussie() -> Band {
    Band::around(Timeframe::Daily, d("0.71500"), d("0.004935"), d("0.46"))
}

/// The same twentieth every band gets.
fn aussie_share() -> Decimal {
    share()
}

/// **One approach is one alert, however much price wobbles inside it.**
///
/// He found this live: *"price approaches a level, price goes back, you keep
/// sending me a message every time... I got so many cards."*
///
/// On this band the way home used to be measured from the band itself — 2.3
/// pips out, at 0.713638 — while *approaching* reaches 4.0 pips, down to
/// 0.713465. **Every price in that 1.7-pip sliver was both "approaching" and
/// "properly gone" at once**, so a wobble smaller than two pips re-armed the
/// alert and fired it again.
///
/// Sampling four prices an hour through August 2026 put it at 45 alerts on
/// this one level in one month. Real ticks arrive about once a second.
#[test]
fn a_wobble_inside_the_approach_does_not_fire_again() {
    let mut watch = Watch::over(vec![aussie()], aussie_share());

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

/// **It still resets when price is genuinely gone**, or the alert would fire
/// once and never again.
#[test]
fn leaving_the_approach_properly_still_re_arms_it() {
    let mut watch = Watch::over(vec![aussie()], aussie_share());

    watch.arrive(d("0.71800"));
    watch.arrive(d("0.71390"));

    // Past approaching AND a tenth of the band beyond it — 0.713238.
    watch.arrive(d("0.71300"));

    assert_eq!(
        watch.arrive(d("0.71390")).len(),
        1,
        "a real visit away makes the next arrival new again"
    );
}
