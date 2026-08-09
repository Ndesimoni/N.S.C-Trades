//! Getting prices in.
//!
//! Keeps the broker connection up, builds finished candles out of incoming
//! prices, saves them, and announces each closed candle to the rest of the
//! system.
//!
//! Reconnects with a delay, and — the part that is easy to forget — **fills in
//! what it missed** before carrying on. Resuming without filling the gap
//! leaves a hole that shifts a swing, which shifts a level, which changes
//! signals for days, with nothing reporting an error anywhere.
//!
//! Candles are announced a moment after they close, so the last price has
//! settled. Signalling on a candle the broker then adjusts is a small,
//! repeatable way to make live results drift away from backtest results.
