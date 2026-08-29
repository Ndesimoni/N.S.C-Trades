//! The numbers that decide what a run of candles is called.
//!
//! **They live in `config/patterns.toml`, not in the code.** Bury a threshold
//! in a function called `is_engulfing` and changing your mind means changing
//! code. Keep it in a file and it is a restart.

use std::path::Path;

use rust_decimal::Decimal;
use serde::Deserialize;
use thiserror::Error;

/// Everything the pattern reading needs.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Rules {
    pub engulfing: Engulfing,
    pub harami: Harami,
    pub tweezers: Tweezers,
    pub piercing: Piercing,
    pub star: Star,
    pub soldiers: Soldiers,
    pub push: Push,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Engulfing {
    /// The least the first candle's body may be, as a share of its candle.
    ///
    /// **Without it almost anything engulfs a doji** and the word stops
    /// meaning anything.
    pub min_first_body: Decimal,

    /// How many times the first body the second must be.
    ///
    /// **Not in the textbook, and spot forex is why.** The textbook rule —
    /// the second body covers the first — assumes a market that gaps. Spot
    /// forex does not: the second candle opens exactly where the first
    /// closed, so one end of the covering is free every single time, and
    /// "engulfing" collapses into "closed past the first candle's open".
    ///
    /// Left alone that found one engulfing every six candles on gold.
    pub min_second_of_first: Decimal,

    /// The least the ENGULFING candle may reach, in normal candles.
    ///
    /// **SHAPE IS NOT SIZE, and both settings above are shape.** They are
    /// shares of the FIRST candle, so a tiny candle swallowing a tinier one
    /// passes them both while nothing has happened.
    ///
    /// Measured 29 August 2026 over 270,000 candles: **39% of engulfings
    /// reached less than one normal candle**, the smallest 0.11 of one. That is
    /// the same hole `[push]` found and plugged with `min_push_reach`.
    ///
    /// **Reach, and deliberately not body.** A reversal at a level has a
    /// rejection wick by nature — his own AUD/USD trigger of 25 August reached
    /// 2.47 normal candles with a 37% body, and a body test would have thrown
    /// away the one candle he pointed at.
    pub min_reach: Decimal,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Harami {
    /// The least the first candle's body may be. If it is not big, nothing is
    /// hiding inside it.
    pub min_first_body: Decimal,

    /// The most the second body may be, as a share of the FIRST body.
    pub max_second_of_first: Decimal,

    /// The least the FIRST, BIG candle may reach, in normal candles.
    ///
    /// **The big one carries the move, so it is the one that has to be big.**
    /// The small candle is the point of the pattern and is left alone.
    ///
    /// Same hole as the engulfing: 38% of haramis had a first candle reaching
    /// less than one normal candle, measured 29 August over 270,000 candles.
    pub min_first_reach: Decimal,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Tweezers {
    /// How close two extremes count as the same price, in normal candles.
    ///
    /// **Never zero.** Two candles do not share a high to the tick.
    pub tolerance_reach: Decimal,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Piercing {
    /// How far into the first body the second must close, as a share of it.
    pub min_close_into_body: Decimal,

    /// The least the first candle's body may be.
    pub min_first_body: Decimal,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Star {
    /// The most of its own candle the middle body may take.
    pub max_middle_body: Decimal,

    /// The least the two outer bodies must take of their own candles.
    pub min_outer_body: Decimal,

    /// How far into the first body the third must close.
    pub min_close_into_body: Decimal,

    /// Must the middle candle have gapped clear of both neighbours?
    ///
    /// **False on spot forex, and that is not a shortcut.** Price runs Sunday
    /// evening to Friday evening without a break, so a candle's open IS the
    /// last one's close. Insist on the gap and the pattern can only form at
    /// the Sunday open, once a week. Whether the gap was there is reported
    /// instead — see `Pattern::Star { abandoned }`.
    pub require_gap: bool,
}

/// Three candles marching the same way.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Soldiers {
    /// The least each of the three bodies must take of its own candle.
    pub min_body: Decimal,

    /// The most the wick AGAINST the move may take, on any of the three.
    ///
    /// **The one test that is not free on spot forex.** The textbook also
    /// asks each candle to open inside the last one's body — but forex opens
    /// exactly ON the last close, so that passes every time and asks nothing.
    /// A long wick against the move means they were pushed back and came
    /// again: a fight, not a march.
    pub max_wick_against: Decimal,
}

/// **His own pattern.** A push, then a pin whose tail opposes it.
///
/// **These are his, not the textbook's.** Every other block in this file is a
/// borrowed default waiting to be replaced. This one was settled with him on
/// 21 August 2026, against real candles on five pairs.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Push {
    /// The least the PUSH candle's body may be, as a share of its own candle.
    pub min_push_body: Decimal,

    /// The least the PUSH candle may reach, in normal candles.
    ///
    /// **The half that gets forgotten.** Body share is shape; this is size.
    /// Without it a quiet candle that happens to be all body counts as
    /// momentum.
    pub min_push_reach: Decimal,

    /// The most of its own candle the PIN's body may take.
    pub max_pin_body: Decimal,

    /// The most the PIN's whole range may be, as a share of the PUSH's range.
    ///
    /// **A pullback that moved further than the push is not a pullback.** The
    /// pin is meant to be price trying to take the move back and failing. If
    /// it covers more ground than the move it is answering, the push was the
    /// smaller event and the shape is telling a different story.
    pub max_pin_of_push: Decimal,

    /// How many times its own body the PIN's tail must be.
    pub min_tail_of_body: Decimal,

    /// The most the wick at the PIN's other end may take of its candle.
    ///
    /// **What keeps indecision out.** Real wick both sides is a spinning top:
    /// nobody won, which is close to the opposite of a refusal.
    pub max_nose: Decimal,
}

/// What can go wrong reading them.
#[derive(Debug, Error)]
pub enum RulesError {
    #[error("could not read {path}: {detail}")]
    CannotRead { path: String, detail: String },

    #[error("{path} is not a set of pattern rules: {detail}")]
    NotRules { path: String, detail: String },
}

/// Read them from a file. **Gives up rather than guessing.**
pub fn load(path: &Path) -> Result<Rules, RulesError> {
    let text = std::fs::read_to_string(path).map_err(|trouble| RulesError::CannotRead {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })?;

    toml::from_str(&text).map_err(|trouble| RulesError::NotRules {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })
}
