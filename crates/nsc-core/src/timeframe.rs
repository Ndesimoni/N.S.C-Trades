//! Timeframes, and the maths that goes with them.
//!
//! Handles: how long each candle lasts, snapping any moment back to the start
//! of its candle, and how many small candles make up a bigger one.
//!
//! The daily candle is the tricky one. In forex, the day does not end at
//! midnight UTC — it ends at the time set in `config/app.toml`, usually 5pm
//! New York. That time decides where every daily level sits, so it is applied
//! here, in one place, instead of being worked out again by whoever needs it.
