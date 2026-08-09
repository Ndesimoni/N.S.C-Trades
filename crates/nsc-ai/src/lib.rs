//! # nsc-ai — the checking layer
//!
//! Two different things live here, and mixing them up is the classic mistake.
//!
//! **The scorer** (`scorer.rs`, Phase 4) — a model trained on the signals you
//! judged. It works on plain numbers, it is easy to inspect, and at a few
//! hundred to a few thousand examples it beats anything fancier. This is the
//! part that learns your judgement.
//!
//! **The AI reviewer** (`validator.rs`, Phase 5) — reads facts the bot has
//! already worked out and gives back a confidence and a list of concerns.
//! A useful second opinion. Not a source of truth.
//!
//! ## The rule
//!
//! **The AI never does arithmetic and never invents a setup. It only filters.**
//!
//! Levels, distances, risk-to-reward and stop placement are worked out by
//! normal code and handed over as finished facts. Ask an AI how far price is
//! from a level and it will give you a confident, believable, wrong number —
//! rarely enough that you will have started trusting it by the time it
//! matters.
//!
//! And it never creates a setup. Your rules find setups. This layer only
//! decides which ones survive.

pub mod client;
pub mod error;
pub mod prompt;
pub mod schema;
pub mod scorer;
pub mod validator;
pub mod vision;
