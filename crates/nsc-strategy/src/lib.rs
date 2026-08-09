//! # nsc-strategy — your rules, in six layers
//!
//! **Never touches the outside world.** Reads a `TaSnapshot`, returns either
//! one setup or nothing.
//!
//! ## The six layers
//!
//! ```text
//!   1. DIRECTION   why am I looking at this pair?    ── must pass
//!   2. PLACE       where must price be?              ── must pass
//!   3. TRIGGER     what makes me click buy?          ── must pass
//!   4. STOP        where does the stop go?           ── works out the SL
//!   5. TARGET      where do I get out?               ── works out the TP
//!   6. SKIP        anything that cancels it anyway?  ── kills the setup
//!
//!   CONFLUENCE     how confident am I?               ── scores it
//! ```
//!
//! ## Why the first three must all pass, instead of being scored
//!
//! Direction, place and trigger are pass-or-fail. Only the extras get points.
//!
//! A pure points system will eventually send you a setup you would never take,
//! and you will have no way to work out which point value caused it. With
//! pass-or-fail gates, every signal can be explained in one sentence — which
//! is also what makes the Telegram message worth reading.
//!
//! ## The first version will disagree with you
//!
//! It will. Your eyes apply filters you have never put into words — an
//! approach into the level that was too steep, a level that got chewed up
//! overnight.
//!
//! **When that happens, a rule is missing from `config/strategy.toml`. It is
//! not a broken model.** Finding those unwritten rules is the whole point of
//! Phase 3, and it is why the 👍/👎 buttons exist before any machine learning
//! does.

pub mod bias;
pub mod confluence;
pub mod engine;
pub mod error;
pub mod invalidation;
pub mod location;
pub mod reasons;
pub mod spec;
pub mod target;
pub mod trigger;
pub mod veto;
