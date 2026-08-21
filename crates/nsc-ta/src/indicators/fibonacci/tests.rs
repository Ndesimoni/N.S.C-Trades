//! The arithmetic, and what it is allowed to conclude.

use std::path::Path;

use rust_decimal::Decimal;

use super::{Leg, Rules, Where, levels, load, read, targets};

fn d(text: &str) -> Decimal {
    text.parse().expect("a number")
}

fn rules() -> Rules {
    load(Path::new("../../config/fibonacci.toml")).expect("config/fibonacci.toml should read")
}

/// Up from 100 to 200. Easy numbers, so a wrong answer is obvious.
fn up() -> Leg {
    Leg::new(d("100"), d("200")).expect("a real move")
}

/// The same move, the other way.
fn down() -> Leg {
    Leg::new(d("200"), d("100")).expect("a real move")
}

/// **A retracement counts back from the extreme, not up from the start.**
///
/// 0.618 of a move up from 100 to 200 is 138.2 — the amount GIVEN BACK. Get
/// this the wrong way round and every level is mirrored: 161.8 instead, which
/// is a real price on the chart and looks perfectly plausible.
#[test]
fn a_retracement_is_measured_back_from_the_extreme() {
    assert_eq!(up().retracement(d("0.618")), d("138.2"));
    assert_eq!(up().retracement(d("0.5")), d("150"));
    assert_eq!(up().retracement(d("0")), d("200"));
    assert_eq!(up().retracement(d("1")), d("100"));
}

/// One formula serves both directions, because the run carries the sign.
#[test]
fn a_move_down_retraces_upward() {
    assert_eq!(down().retracement(d("0.618")), d("161.8"));
    assert_eq!(down().retracement(d("0.5")), d("150"));
}

/// **An extension goes BEYOND the extreme**, and is not a retracement with a
/// number over one.
///
/// 1.272 of that move up is 227.2. Run it through the retracement formula
/// instead and you get −27.2, which is off the bottom of the chart — the same
/// mistake gives a plausible-looking number on a real pair.
#[test]
fn an_extension_goes_past_the_extreme() {
    assert_eq!(up().extension(d("1.272")), d("227.2"));
    assert_eq!(up().extension(d("1.618")), d("261.8"));
    assert_eq!(down().extension(d("1.272")), d("72.8"));
}

/// Depth is a share, so it reads the same on gold and on the euro.
#[test]
fn depth_is_a_share_of_the_move() {
    assert_eq!(up().how_deep(d("200")), d("0"));
    assert_eq!(up().how_deep(d("150")), d("0.5"));
    assert_eq!(up().how_deep(d("100")), d("1"));
    assert_eq!(down().how_deep(d("150")), d("0.5"));
}

/// **A move of nothing is not a move.** Every share of it would be nothing,
/// and dividing by it makes every answer meaningless without saying so.
#[test]
fn a_move_that_did_not_move_is_refused() {
    assert!(Leg::new(d("100"), d("100")).is_none());
}

/// The four readings, in the order price passes through them.
#[test]
fn price_is_read_by_how_deep_it_has_come() {
    let rules = rules();
    let at = |price: &str| read(up(), d(price), &rules);

    // The boundaries on this move: 161.8 is 0.382 back, 150 is a half,
    // 138.2 is 0.618, and 121.4 is 0.786.
    assert_eq!(at("210"), Where::StillGoing);
    assert_eq!(at("200"), Where::BarelyPaused);
    assert_eq!(at("175"), Where::BarelyPaused);
    assert_eq!(at("155"), Where::ComingBack);
    assert_eq!(at("150"), Where::GoldenZone);
    assert_eq!(at("138.2"), Where::GoldenZone);
    assert_eq!(at("130"), Where::Deeper);
    assert_eq!(at("110"), Where::PastTheStop);
    assert_eq!(at("90"), Where::Undone);
}

/// **The shallow one is a reading about the MOVE, not an entry.**
///
/// A pullback that stops above 0.382 means the market barely paused, which is
/// what a powerful move looks like. Reading it as "not deep enough to buy"
/// throws away the only thing it was telling you.
#[test]
fn barely_paused_is_its_own_answer() {
    let rules = rules();

    // 0.3 deep — shallower than the strong-trend level.
    assert_eq!(read(up(), d("170"), &rules), Where::BarelyPaused);
    assert_eq!(Where::BarelyPaused.spoken(), "barely paused");
}

/// **Four lines, and only four.** A level with no job attached is a line the
/// bot draws and nothing reads.
#[test]
fn exactly_four_levels_get_drawn() {
    let drawn = levels(up(), &rules());

    assert_eq!(drawn.len(), 4);

    let prices: Vec<Decimal> = drawn.iter().map(|(_, price)| *price).collect();
    assert_eq!(prices, vec![d("161.8"), d("150"), d("138.2"), d("121.4")]);
}

/// They come out in the order they sit on the chart, deepest last.
#[test]
fn the_levels_come_out_in_chart_order() {
    let drawn = levels(up(), &rules());
    let shares: Vec<Decimal> = drawn.iter().map(|(share, _)| *share).collect();

    assert_eq!(shares, vec![d("0.382"), d("0.5"), d("0.618"), d("0.786")]);
}

/// Targets sit beyond the extreme, and are the standard numbers rather than
/// his — which is written down in the config next to them.
#[test]
fn the_targets_are_past_the_extreme() {
    let aimed = targets(up(), &rules());

    assert_eq!(aimed.len(), 2);
    assert!(aimed.iter().all(|(_, price)| *price > up().to()));
}
