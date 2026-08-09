//! # nsc-chart — turning a setup into a picture
//!
//! Draws the chart that goes with every signal.
//!
//! This is not decoration. The image is what lets you judge a setup in two
//! seconds on your phone, and judging setups quickly is the entire reason the
//! 👍/👎 loop produces any training data. A signal you have to open a laptop
//! to evaluate is a signal you will not evaluate.
//!
//! It also doubles as debugging. When the bot draws a level you disagree with,
//! seeing the level drawn is much faster than reading the swing points that
//! produced it.

pub mod error;
pub mod overlays;
pub mod render;
pub mod theme;
