//! Saving what **you** thought of each signal.
//!
//! Written from two places: the 👍/👎 buttons in Telegram, and the chart replay
//! tool in Phase 4.
//!
//! Your verdict and the actual result answer different questions, and you need
//! both. The result says whether the trade won. Your verdict says whether you
//! would have taken it. A model trained only on results learns to chase
//! winners, including ones you would never have entered. Trained on both, it
//! learns your judgement — which is the actual goal.
//!
//! The `note` field matters more than it looks. Every skip you cannot explain
//! with an existing rule is a rule missing from `config/strategy.toml`.
