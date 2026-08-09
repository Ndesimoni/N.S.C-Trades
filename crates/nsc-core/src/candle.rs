//! One candle, and groups of them.
//!
//! `open_time` is when the candle **started**, always in UTC. Storing the
//! close time instead causes off-by-one-candle bugs that are painful to spot
//! in a backtest.
//!
//! `complete` says whether the candle has finished forming. Analysis must
//! ignore unfinished candles. Acting on a half-formed candle is a quiet form
//! of using data you do not have yet, because the candle you signalled on is
//! not the candle that ends up in the history.
