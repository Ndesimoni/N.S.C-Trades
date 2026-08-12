//! The settings this tool runs with.
//!
//! Written out here rather than read from `config/ta.toml`, because loading
//! the config is Phase 0 work that does not exist yet. When it does, this file
//! reads it.
//!
//! Until then these are copied by hand and could drift from the real file —
//! which is exactly why this is a scratch tool for looking at a chart and not
//! part of the bot.

use nsc_ta::config::{
    CandleSettings, FibSettings, LevelSettings, StructureSettings, SwingSettings,
};

use crate::Args;

pub fn swing_settings(args: &Args) -> SwingSettings {
    SwingSettings {
        confirm_retracement: 0.5,
        shallow_retracement: 0.382,
        min_run_fraction: args.min_run_fraction,
        run_memory_legs: args.run_memory_legs,
    }
}

pub fn level_settings() -> LevelSettings {
    LevelSettings {
        band_atr_multiple: 0.5,
        min_touches: 2,
        max_age_bars: 500,
    }
}

pub fn structure_settings() -> StructureSettings {
    StructureSettings {
        min_follow_through: 0.4,
    }
}

pub fn fib_settings() -> FibSettings {
    FibSettings {
        golden_zone: [0.5, 0.618],
        strong_trend_level: 0.382,
        stop_level: 0.786,
        extensions: [1.272, 1.618],
    }
}

pub fn candle_settings() -> CandleSettings {
    CandleSettings {
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
    }
}
