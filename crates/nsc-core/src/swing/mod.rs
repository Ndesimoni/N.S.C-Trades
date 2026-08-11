//! Swing highs and lows — the building block for everything else.
//!
//! Levels, trendlines, Fibonacci anchors, trend direction and chart patterns
//! are **all** worked out from swing points. Nothing else in this codebase
//! affects as much.
//!
//! ## The two times, and why they are separate
//!
//! A swing high at candle 100 is not *known* to be a swing high until candle
//! 103 or so — you need to see price come back down first. Using it any
//! earlier means using knowledge you did not have, and every backtest built
//! on that is fiction.
//!
//! So a swing carries `bar_time` (where it sits on the chart) and
//! `confirmed_at` (when you could first know about it). Keeping them apart
//! makes it hard to forget the difference.
//!
//! Scroll back over any chart and the highs are obvious. That is the trap. At
//! the moment candle 100 printed, nobody knew it was a high — price could
//! have carried on up. It only became one once price turned away.
//!
//! [`Swing::is_known_at`] is what keeps that honest. Call it before using a
//! swing for anything.
//!
//! ## What is where
//!
//! - [`kind`] — high or low
//! - [`point`] — the swing itself

mod kind;
mod point;

#[cfg(test)]
mod tests;

pub use kind::SwingKind;
pub use point::Swing;
