//! Bullish and bearish engulfing candles.
//!
//! The body of this candle completely covers the body of the last one. Bodies
//! only — including the wicks makes it fire far too often to be useful.
//!
//! Quality is scored on: how big the engulfing body is compared to a normal
//! candle, whether it closed near the extreme of its range, and whether the
//! candle it swallowed actually pointed the other way. A big engulfing candle
//! that closes in the middle is much weaker than the name suggests.
