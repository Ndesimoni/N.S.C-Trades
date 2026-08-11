use super::*;
use crate::error::TaError;

/// The values actually in config/ta.toml today.
fn settings() -> TaSettings {
    TaSettings {
        swings: SwingSettings {
            lookback: 3,
            require_confirmed: true,
            min_atr_multiple: 0.5,
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

#[test]
fn a_lookback_of_zero_is_refused() {
    let mut s = settings();
    s.swings.lookback = 0;

    assert!(matches!(s.validate(), Err(TaError::BadSetting { .. })));
}

#[test]
fn a_negative_noise_filter_is_refused() {
    let mut s = settings();
    s.swings.min_atr_multiple = -1.0;

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
        lookbak = 3
        require_confirmed = true
        min_atr_multiple = 0.5
    "#;

    assert!(toml::from_str::<SwingSettings>(toml).is_err());
}
