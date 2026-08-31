//! Firing once per touch, not once per price.

use rust_decimal::Decimal;

use super::super::{AtZone, Band, Nearness, Timeframe, Watch, nearness};
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

// Leaving is a REAL distance. A pip back out must not reset the band, or the
// next pip back in is a second alert for one visit.
#[test]
fn once_it_has_spoken_it_stays_quiet() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(watch.arrive(d("4132.60")).len(), 1, "the one card");

    // Hovering at the edge.
    assert!(watch.arrive(d("4143")).is_empty(), "not news");
    assert!(watch.arrive(d("4132.60")).is_empty(), "nor is coming back");

    // **Properly gone, and it still says nothing.** Leaving used to re-arm the
    // card; on 31 August he asked for silence until a candle CLOSES somewhere
    // new. `clear_of` still resets the WORDING for the next visit — see
    // `Level::deepest` — but not what the level has said.
    watch.arrive(d("4145"));
    assert!(
        watch.arrive(d("4132.60")).is_empty(),
        "a real trip away is still not a new card"
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

/// **A close is the only thing that follows an approach.**
///
/// His sequence, 31 August 2026: approaching, then the candle closes below,
/// then silence however often price comes back — until a candle closes ABOVE.
#[test]
fn a_close_speaks_and_then_only_a_different_close_does() {
    let mut watch = Watch::over(vec![aussie()], aussie_share());
    let band = aussie();

    watch.arrive(d("0.71800"));
    assert_eq!(watch.arrive(d("0.71390")).len(), 1, "approaching");

    // The candle closes below. New news, so it speaks.
    assert!(
        watch.closed(&band, "4h", AtZone::ClosedBelow),
        "closed below"
    );

    // Later candles come back to the level. Silence — his exact complaint.
    watch.arrive(d("0.71300"));
    assert!(
        watch.arrive(d("0.71390")).is_empty(),
        "coming back is silent"
    );
    assert!(watch.arrive(d("0.71390")).is_empty(), "and again");

    // Another close below is the same ending, so it says nothing either.
    assert!(
        !watch.closed(&band, "4h", AtZone::ClosedBelow),
        "the same ending twice is not news"
    );

    // A close ABOVE is a different ending. This he wants.
    assert!(
        watch.closed(&band, "4h", AtZone::ClosedAbove),
        "closed above"
    );
}

/// **Each timeframe keeps its own story about a level.**
///
/// A 4-hour candle closing below a weekly level and a daily candle doing the
/// same are two different pieces of news about one line, and the daily is the
/// bigger one. One shared memory would let whichever arrived first silence the
/// other.
#[test]
fn the_daily_and_the_four_hour_do_not_silence_each_other() {
    let mut watch = Watch::over(vec![aussie()], aussie_share());
    let band = aussie();

    assert!(watch.closed(&band, "4h", AtZone::ClosedBelow));
    assert!(
        watch.closed(&band, "1d", AtZone::ClosedBelow),
        "the daily has not said this yet"
    );

    // But each still keeps quiet about repeating itself.
    assert!(!watch.closed(&band, "4h", AtZone::ClosedBelow));
    assert!(!watch.closed(&band, "1d", AtZone::ClosedBelow));
}

/// **Any close on any timeframe ends the approach card for good.**
///
/// Price being near a line stops being news the moment the line has a story.
#[test]
fn a_close_on_any_timeframe_silences_the_approach() {
    let mut watch = Watch::over(vec![aussie()], aussie_share());
    let band = aussie();

    // A daily close, before price has ever been reported as approaching.
    watch.closed(&band, "1d", AtZone::ClosedAbove);

    watch.arrive(d("0.71800"));
    assert!(
        watch.arrive(d("0.71390")).is_empty(),
        "the level already has a story"
    );
}
