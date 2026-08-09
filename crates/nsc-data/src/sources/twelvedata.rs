//! Twelve Data (or a similar vendor).
//!
//! Straightforward web requests with an API key. Useful when your data
//! provider and your broker are deliberately different, or when your broker's
//! history is too short to backtest properly.
//!
//! One caveat: vendor prices will not match your broker exactly. For reading
//! structure on higher timeframes that does not matter. For anything
//! spread-sensitive it does.
