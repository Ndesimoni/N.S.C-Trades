//! Saving and loading candles.
//!
//! Two very different access patterns:
//!   - live: fetch the last few hundred candles for one pair, constantly
//!   - backtest: read years of candles forward in time, once per run
//!
//! The backtest one must **stream** rather than load everything into memory.
//! Twenty pairs of 15-minute candles over five years is fine today, but a
//! settings sweep runs that hundreds of times, and how much memory it uses
//! decides whether sweeps are comfortable or painful.
//!
//! Writes are safe to repeat. Running a download twice repairs the history
//! instead of duplicating it.
