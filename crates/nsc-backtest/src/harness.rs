//! Putting one backtest run together and recording it.
//!
//! Loads the settings, builds the engine, replays the period, follows every
//! signal to its result, works out the statistics, and saves the run with its
//! full settings snapshot and code version.
//!
//! Results are worked out on the **small** candles, not the signal's
//! timeframe. On an hourly signal, checking only hourly candles cannot tell
//! you whether the stop or the target got hit first within that hour — and
//! guessing in your own favour is one of the most common ways a backtest
//! quietly overstates itself.
//!
//! Where even 15-minute candles cannot separate them, the result is recorded
//! as `ambiguous` and left out of the numbers.
