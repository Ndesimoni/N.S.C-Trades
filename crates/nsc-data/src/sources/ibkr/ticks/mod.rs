//! The live price line — one subscription per pair, folded into one channel.
//!
//! **IBKR never sends a price.** It sends a bid, and separately an ask, and
//! the middle of the two is worked out here. That is not a detail: the candles
//! come back as `MidPoint`, so a live price taken from either side would be
//! measured against bands drawn on something else.
//!
//! ```text
//!     spread.rs     the last bid, the last ask, and the middle
//!     listening.rs  subscribing to every pair, and what each tick means
//! ```

mod listening;
mod spread;

#[cfg(test)]
mod tests;

pub(super) use listening::open;
