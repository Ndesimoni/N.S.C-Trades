//! Average True Range — the most important number in this codebase.
//!
//! ATR is simply how big a normal candle is right now. It predicts nothing.
//! It matters because it is the **yardstick**.
//!
//! How close counts as "at the level", how far apart swings must be to count
//! as separate levels, how much room the stop gets, how big a candle is too
//! big to enter on — all of it is measured in ATR.
//!
//! That is what lets one settings file work on EURUSD and GBPJPY at once. A
//! system tuned in pips has to be retuned for every pair and every change in
//! volatility. A system tuned in ATR mostly does not.
