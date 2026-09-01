//! **How close counts as being at a band.**
//!
//! A share of that band's own thickness, so it means the same on gold as on
//! the euro. It was one price per pair until 31 August 2026 — four pips, which
//! is 22% of an AUD/USD daily band and 0.03% of a gold weekly one.
//!
//! Only the report made when watching resumes reads this now. Price reaching a
//! level stopped being a message on the same day.

use rust_decimal::Decimal;

use super::super::{Band, Nearness, Timeframe, Watch, nearness};
use super::support::{aussie, d, gold, share};

/// This band's own reach.
fn reach() -> Decimal {
    gold().thickness() * share()
}

// The band's top is 4132.57, and a twentieth of it is $3.86.
#[test]
fn the_slack_is_a_twentieth_and_no_more() {
    let band = gold();

    let grazing = nearness(&band, d("4132.60"), reach());
    assert_eq!(grazing, Nearness::Approaching);
    assert_eq!(nearness(&band, d("4132.50"), reach()), Nearness::Inside);

    // A dollar out is still a touch. It was not when this setting was ten
    // cents, and gold is why it changed.
    let dollar_out = nearness(&band, d("4133.60"), reach());
    assert_eq!(dollar_out, Nearness::Approaching);

    assert_eq!(nearness(&band, d("4140"), reach()), Nearness::Away);
}

// The same twentieth on a band 340 times thinner. This is the whole point of
// making it a share: 0.71613 + 22.7 pips / 20 is about 0.71624.
#[test]
fn the_same_share_travels_to_a_thin_band() {
    let band = aussie();
    let reach = band.thickness() * share();

    assert_eq!(nearness(&band, d("0.71620"), reach), Nearness::Approaching);
    assert_eq!(nearness(&band, d("0.71600"), reach), Nearness::Inside);
    assert_eq!(nearness(&band, d("0.71700"), reach), Nearness::Away);
}

#[test]
fn only_the_band_price_is_at_is_resting() {
    let daily = Band::around(Timeframe::Daily, d("2984"), d("70.36"), d("0.46"));
    let mut watch = Watch::over(vec![gold(), daily], share());

    watch.saw(d("4100"));

    let resting = watch.resting_at();
    assert_eq!(resting.len(), 1);
    assert_eq!(resting[0].timeframe, Timeframe::Weekly);
}

#[test]
fn a_price_near_nothing_rests_nowhere() {
    let mut watch = Watch::over(vec![gold()], share());

    watch.saw(d("4280"));
    assert!(watch.resting_at().is_empty());
}
