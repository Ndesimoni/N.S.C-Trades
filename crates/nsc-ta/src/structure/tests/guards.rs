//! Does it refuse what it should refuse?

use nsc_core::candle::Candle;

use super::helpers::*;
use crate::config::StructureSettings;
use crate::error::TaError;
use crate::structure::StructureReader;

// ── Guards ──

#[test]
fn every_break_comes_after_the_extreme_it_broke() {
    let breaks = breaks_on(&[100, 200, 300, 250, 200, 260, 320, 380, 300, 200, 100]);

    for broken in &breaks {
        assert!(broken.at() > broken.broken_at(), "{broken:?}");
    }
}

#[test]
fn an_unfinished_candle_is_refused() {
    let still_forming = Candle::new(
        at(0),
        price(100),
        price(105),
        price(95),
        price(100),
        None,
        false,
    )
    .expect("valid candle");

    let mut reader = StructureReader::new(settings()).expect("valid settings");

    assert!(matches!(
        reader.update(&still_forming, &[]),
        Err(TaError::IncompleteCandle { .. })
    ));
}

// With no follow-through required, a one-point poke past an old high would
// count as taking it — which is the whole thing this module exists to refuse.
#[test]
fn asking_for_no_follow_through_is_refused() {
    let broken = StructureSettings {
        min_follow_through: 0.0,
    };

    assert!(matches!(
        StructureReader::new(broken),
        Err(TaError::BadSetting { .. })
    ));
}
