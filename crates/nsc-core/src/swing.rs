//! Swing highs and lows — the building block for everything else.
//!
//! Levels, trendlines, Fibonacci anchors, trend direction and chart patterns
//! are **all** worked out from swing points. Nothing else in this codebase
//! affects as much.
//!
//! The field that matters is `confirmed_at`.
//!
//! A swing high at candle 100 is not *known* to be a swing high until candle
//! 103 or so — you need to see price come back down first. Using it any
//! earlier means using knowledge you did not have, and every backtest built on
//! that is fiction.
//!
//! So the type carries two separate times: `bar_time` (where the swing is) and
//! `confirmed_at` (when you could first know about it). Keeping them apart
//! makes it hard to forget the difference.
