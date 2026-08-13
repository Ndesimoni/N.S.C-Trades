//! Tests for the level finder.
//!
//! - [`helpers`] — building charts and swings to test with
//! - [`grouping`] — where the band ends up when you slide it
//! - [`detection`] — does it draw the levels you would draw
//! - [`absorbing`] — which levels lose their line to a bigger timeframe
//! - [`guards`] — does it refuse what it should refuse

mod absorbing;
mod detection;
mod grouping;
mod guards;
mod helpers;
