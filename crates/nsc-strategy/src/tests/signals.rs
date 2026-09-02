//! End to end: a real shape, at a real level.

use super::support::{band, d, his_gold, rules};
use crate::shape::{Traded, traded};
use crate::{Standing, look, place::Placing, reasons};
use nsc_core::candle::Bar;
use nsc_core::levels::{Band, Timeframe};
use nsc_ta::pattern::{self, Pattern};

fn patterns() -> pattern::Rules {
    pattern::load(std::path::Path::new("../../config/patterns.toml"))
        .expect("config/patterns.toml should read")
}

// ── which shapes he trades ─────────────────────────────────────────────────

#[test]
fn his_own_and_the_engulfings_count() {
    assert_eq!(
        traded(Pattern::Push { up: true }),
        Some(Traded::Push { up: true })
    );
    assert_eq!(
        traded(Pattern::Engulfing { up: false }),
        Some(Traded::Engulfing { up: false })
    );

    // Added 29 August 2026, at his word.
    assert_eq!(
        traded(Pattern::Harami { up: true }),
        Some(Traded::Harami { up: true })
    );
    assert_eq!(
        traded(Pattern::Marching { up: false }),
        Some(Traded::Marching { up: false })
    );
}

/// **Eight shapes are named in `pattern/` and he trades four.** The rest are
/// there because they are on every candlestick page, not because they are on
/// his chart — and a detector firing on eight is twice the messages and half
/// the meaning.
///
/// **Harami and marching moved OUT of this list on 29 August 2026.** They are
/// his now; the four below never were.
#[test]
fn the_textbook_rest_do_not() {
    assert_eq!(traded(Pattern::Tweezer { top: true }), None);
    assert_eq!(traded(Pattern::PiercingLine), None);
    assert_eq!(traded(Pattern::DarkCloudCover), None);
    assert_eq!(
        traded(Pattern::Star {
            up: true,
            abandoned: false
        }),
        None
    );
}

/// **The tail tip, and it is argued from what the pattern is.** The tail is a
/// pullback that failed; if it reached the level, the level is what stopped
/// it. Measuring from the body would pass a shape whose rejection happened
/// somewhere else entirely.
#[test]
fn a_push_is_measured_from_its_tail_tip() {
    let (bars, _) = his_gold();
    let seen: Vec<&Bar> = bars.iter().collect();
    let pin = &bars[1];

    // Push up, so the tail points down: the pin's low.
    assert_eq!(Traded::Push { up: true }.touching(&seen), Some(pin.low));

    // Push down, so the tail points up: the pin's high.
    assert_eq!(Traded::Push { up: false }.touching(&seen), Some(pin.high));
}

/// An engulfing has no tail to speak of, so it is measured from where the
/// second candle settled — which is what "engulfing at the level" means to
/// the eye.
#[test]
fn an_engulfing_is_measured_from_its_close() {
    let (bars, _) = his_gold();
    let seen: Vec<&Bar> = bars.iter().collect();
    let second = &bars[1];

    assert_eq!(
        Traded::Engulfing { up: true }.touching(&seen),
        Some(second.close)
    );
}

/// **A harami is measured from its BIG candle, not its small one.**
///
/// Price travelled into the zone on the big candle; the small one only proves
/// it stopped there. Measuring from the small candle would put the setup a
/// whole candle's range away from the level that caused it — and on gold that
/// is nearly 200 points.
#[test]
fn a_harami_is_measured_from_its_first_big_candle() {
    let (bars, _) = his_gold();
    let seen: Vec<&Bar> = bars.iter().collect();
    let big = &bars[0];

    // Bullish: the big candle fell into the zone, so its low is the reach.
    assert_eq!(Traded::Harami { up: true }.touching(&seen), Some(big.low));

    // Bearish: it rose into the zone, so its high is.
    assert_eq!(Traded::Harami { up: false }.touching(&seen), Some(big.high));
}

/// **A march is measured from where it STARTED.**
///
/// Three candles launch from somewhere, and if a zone is there that is the
/// zone they broke out of. Measuring from the end would report the setup three
/// candles away from the level that explains it.
#[test]
fn a_march_is_measured_from_the_candle_it_started_on() {
    let (two, _) = his_gold();
    let mut bars = vec![two[0].clone()];
    bars.extend(two.clone());
    let seen: Vec<&Bar> = bars.iter().collect();

    assert_eq!(
        Traded::Marching { up: true }.touching(&seen),
        Some(bars[0].low)
    );
    assert_eq!(
        Traded::Marching { up: false }.touching(&seen),
        Some(bars[0].high)
    );
}

/// **Too few candles is nothing, not a near miss.**
///
/// A march needs three. Handed two it must give back nothing rather than
/// reaching for an index that is not there or quietly measuring from the wrong
/// candle — which would read as a setup at a level nowhere near it.
#[test]
fn a_shape_handed_too_few_candles_gives_back_nothing() {
    let (bars, _) = his_gold();
    let seen: Vec<&Bar> = bars.iter().collect();

    assert_eq!(Traded::Marching { up: true }.touching(&seen), None);
    assert_eq!(Traded::Marching { up: false }.touching(&seen), None);

    // And nothing at all is nothing, for every shape.
    assert_eq!(Traded::Push { up: true }.touching(&[]), None);
}

// ── the whole thing ────────────────────────────────────────────────────────

/// **His own gold, and a zone under it.** The pin's low is 4450.71, which sits
/// inside a band running 4450 to 4550.
#[test]
fn his_gold_at_a_zone_is_a_signal() {
    let (bars, normal) = his_gold();
    let borrowed: Vec<&_> = bars.iter().collect();

    let found = look(&borrowed, &[band()], normal, &patterns(), &rules())
        .expect("his own pattern, sitting in a zone");

    assert_eq!(found.shape, Traded::Push { up: true });
    assert_eq!(found.standing.placing(), Placing::Inside);
    assert!(
        found.standing.solid(),
        "in the zone is the tier that asks to act"
    );
}

/// **The same run with the zone moved away is not a setup.** This is the whole
/// rung in one test: the shape is identical and the level is what decides.
///
/// **Size has nothing to do with it.** The test below proves the same run at
/// 3.99 normal candles is equally silent with no zone under it.
#[test]
fn the_same_shape_with_no_zone_under_it_says_nothing() {
    let (bars, normal) = his_gold();
    let borrowed: Vec<&_> = bars.iter().collect();

    let elsewhere = Band {
        timeframe: Timeframe::Weekly,
        price: d("3000"),
        top: d("3050"),
        bottom: d("2950"),
    };

    assert!(
        look(&borrowed, &[elsewhere], normal, &patterns(), &rules()).is_err(),
        "a shape a thousand points from any level is not a setup"
    );
}

#[test]
fn no_zones_at_all_is_no_signal() {
    let (bars, normal) = his_gold();
    let borrowed: Vec<&_> = bars.iter().collect();

    assert!(look(&borrowed, &[], normal, &patterns(), &rules()).is_err());
}

/// **A shape away from every zone says nothing, however big it is.**
///
/// A `Bold` tier existed for one day — 2x a normal candle with no level under
/// it — and he took it out on 30 August: *"remove the signals that do not form
/// in our zone and keep only the signals that form in our zone and close to
/// our zone."*
///
/// The same gold run judged against a normal candle of 50 reaches 3.99 of one,
/// which is as bold as this project ever measured. **With no zone under it, it
/// is still silence.**
#[test]
fn no_zone_means_no_signal_however_big_the_shape() {
    let (bars, _) = his_gold();
    let borrowed: Vec<&_> = bars.iter().collect();

    assert!(
        look(&borrowed, &[], d("50"), &patterns(), &rules()).is_err(),
        "3.99 normal candles with no level under it is still not a setup"
    );
}

/// **A zone is what makes a shape worth anything**, and the same bold run with
/// one under it is a signal.
#[test]
fn the_same_run_at_a_zone_is_a_signal() {
    let (bars, _) = his_gold();
    let borrowed: Vec<&_> = bars.iter().collect();

    let found =
        look(&borrowed, &[band()], d("50"), &patterns(), &rules()).expect("bold AND at a zone");

    assert!(matches!(found.standing, Standing::Inside { .. }));
    assert!(found.reach > d("2.0"));
}

/// A close outside the band is **reported, never required**. He was asked
/// whether the break was the trigger and said the shape at the zone is a
/// signal either way.
#[test]
fn breaking_out_is_reported_not_required() {
    let (bars, normal) = his_gold();
    let borrowed: Vec<&_> = bars.iter().collect();

    // The pin closes at 4515.78, inside a band of 4450 to 4550.
    let held = look(&borrowed, &[band()], normal, &patterns(), &rules()).expect("a signal");
    assert!(!held.standing.broke_out());

    // The same run against a band it closed above.
    let below = Band {
        timeframe: Timeframe::Daily,
        price: d("4460"),
        top: d("4480"),
        bottom: d("4440"),
    };

    let broke = look(&borrowed, &[below], normal, &patterns(), &rules())
        .expect("the tail still reaches the band");

    assert!(
        broke.standing.broke_out(),
        "4515.78 closed above a band topping at 4480"
    );
}

// ── the sentence ───────────────────────────────────────────────────────────

/// **Every signal must be explainable in one sentence.** If it cannot be
/// written, the rules are too loose — that is a `CLAUDE.md` rule and it is a
/// test of the rules, not of the wording.
#[test]
fn the_signal_explains_itself_in_one_line() {
    let (bars, normal) = his_gold();
    let borrowed: Vec<&_> = bars.iter().collect();

    let found = look(&borrowed, &[band()], normal, &patterns(), &rules()).expect("a signal");
    let said = reasons::sentence(&found, "XAU/USD", "1d", 2);

    assert_eq!(said, "nsc-bull on XAU/USD 1d, in your weekly 4500 zone");
}

/// **It never says buy or sell.** Where the stop goes has not been settled, and
/// a signal with no stop is a reading rather than a trade.
#[test]
fn nothing_it_says_is_an_instruction() {
    let (bars, normal) = his_gold();
    let borrowed: Vec<&_> = bars.iter().collect();

    let found = look(&borrowed, &[band()], normal, &patterns(), &rules()).expect("a signal");

    for words in [
        reasons::headline(&found),
        reasons::sentence(&found, "XAU/USD", "1d", 2),
    ] {
        let plain = words.to_lowercase();

        for banned in ["buy", "sell", "entry", "target", "stop"] {
            assert!(
                !plain.contains(banned),
                "a version-1 signal must not say {banned:?}: {words}"
            );
        }
    }
}
