//! Catching the use of data you did not have yet. **The most important file
//! here.**
//!
//! Wraps whatever the engine can see and crashes loudly, in test builds, if
//! the analysis touches:
//!   - a candle later than the one being analysed
//!   - a swing before the candle that confirmed it
//!   - a candle that has not finished forming
//!   - a bigger-timeframe candle whose period is not over
//!
//! Why this gets its own file instead of being a habit during code review:
//!
//! **This mistake does not produce an error. It produces a better result.**
//!
//! The backtest finishes. The equity curve looks excellent. The only symptom
//! is that live trading never resembles it — and by then months have gone by
//! and the cause is several rewrites back.
//!
//! Anything that survives a run with these checks on is at least achievable.
//! That is a much stronger statement than "the numbers look good".
