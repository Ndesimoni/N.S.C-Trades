//! Tests for building bigger candles.
//!
//! - [`helpers`] — 15-minute candles anchored to a real daily close
//! - [`building`] — do the bigger candles come out right
//! - [`guards`] — is one ever handed out before it has finished

mod building;
mod guards;
mod helpers;
