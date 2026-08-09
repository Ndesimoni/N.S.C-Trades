//! Layer 1 — which way am I allowed to trade this pair?
//!
//! Looks at the higher timeframe (trend structure, or a moving average, or
//! where price sits in the day's range) and answers Long, Short, or Neither.
//! "Neither" means no setup can be produced at all, however good the rest of
//! it looks.
//!
//! This is the cheapest filter there is and the one most often skipped.
//! Without it the bot sends a buy and a sell on the same pair in the same
//! session — and once you have caught it contradicting itself, you stop
//! reading it.
//!
//! `allow_counter_trend` exists for traders who fade extremes at big levels.
//! If you do that, make it a **separate strategy file** with its own numbers.
//! Fading and following have different win rates and different risk-to-reward,
//! and averaging them into one set of statistics hides both.
