use super::*;
use crate::error::TaError;

/// The values actually in config/ta.toml today.
fn settings() -> TaSettings {
    TaSettings {
        swings: SwingSettings {
            confirm_retracement: 0.5,
            shallow_retracement: 0.382,
            min_run_fraction: 0.5,
            run_memory_legs: 5,
        },
        levels: LevelSettings {
            band_atr_multiple: 0.5,
            min_touches: 2,
            max_age_bars: 500,
            absorb_gap_bands: 1.5,
            min_separation_bands: 3.0,
        },
        structure: StructureSettings {
            min_follow_through: 0.4,
        },
        candles: CandleSettings {
            pin_min_tail_to_body: 2.0,
            pin_max_body_share: 0.33,
            pin_max_nose_share: 0.25,
            engulfing_min_first_body_share: 0.1,
            doji_max_body_share: 0.05,
            doji_max_missing_wick_share: 0.05,
            belt_hold_max_open_wick_share: 0.05,
            belt_hold_min_body_share: 0.6,
            belt_hold_min_atr_multiple: 1.0,
            tweezer_tolerance_atr: 0.05,
            star_max_middle_body_share: 0.2,
            star_min_outer_body_share: 0.5,
            star_min_close_into_first: 0.5,
        },
        fibonacci: FibSettings {
            golden_zone: [0.5, 0.618],
            strong_trend_level: 0.382,
            stop_level: 0.786,
            extensions: [1.272, 1.618],
        },
        indicators: IndicatorSettings {
            atr_period: 14,
            rsi_period: 14,
        },
    }
}

#[test]
fn the_settings_in_ta_toml_are_accepted() {
    assert_eq!(settings().validate(), Ok(()));
}

// Every swing setting is a share of a move. A number above one means somebody
// typed a price or a pip count into a field that wanted a fraction.
#[test]
fn a_give_back_of_more_than_the_whole_run_is_refused() {
    let mut s = settings();
    s.swings.confirm_retracement = 1.5;

    assert!(matches!(s.validate(), Err(TaError::BadSetting { .. })));
}

#[test]
fn a_negative_share_is_refused() {
    let mut s = settings();
    s.swings.min_run_fraction = -1.0;

    assert!(matches!(s.validate(), Err(TaError::BadSetting { .. })));
}

// If the shallow route needed a deeper pullback than the one that confirms on
// its own, it could never fire, and the strong-trend case it exists for would
// silently stop working.
#[test]
fn a_shallow_route_deeper_than_the_deep_one_is_refused() {
    let mut s = settings();
    s.swings.shallow_retracement = 0.8;

    assert!(matches!(s.validate(), Err(TaError::BadSetting { .. })));
}

#[test]
fn a_band_with_no_thickness_is_refused() {
    let mut s = settings();
    s.levels.band_atr_multiple = 0.0;

    assert!(matches!(s.validate(), Err(TaError::BadSetting { .. })));
}

// One swing point is a swing point. It takes two for a price to have turned
// the market more than once, which is the whole idea of a level.
#[test]
fn a_level_needing_only_one_touch_is_refused() {
    let mut s = settings();
    s.levels.min_touches = 1;

    assert!(matches!(s.validate(), Err(TaError::BadSetting { .. })));
}

#[test]
fn an_atr_period_of_one_is_refused() {
    let mut s = settings();
    s.indicators.atr_period = 1;

    assert!(matches!(s.validate(), Err(TaError::BadSetting { .. })));
}

// A typo in ta.toml must stop the program, not be quietly ignored. If it were
// ignored, you would change a setting, see no difference in the results, and
// spend an evening working out why.
#[test]
fn a_misspelled_setting_is_refused() {
    let toml = r#"
        confirm_retracemnt  = 0.5
        shallow_retracement = 0.382
        min_run_fraction    = 0.5
        run_memory_legs     = 5
    "#;

    assert!(toml::from_str::<SwingSettings>(toml).is_err());
}
