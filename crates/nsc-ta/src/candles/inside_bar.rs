//! Inside bars, and the breakout from them.
//!
//! An inside bar means the market is coiling. It has no direction of its own —
//! which is why the thing you trade is the **break** of the candle before it,
//! not the inside bar itself.
//!
//! This module tracks runs of them too. Two or three inside bars stacked up
//! coil tighter and break harder. It reports the break with a direction, so
//! the trigger layer has something it can act on.
