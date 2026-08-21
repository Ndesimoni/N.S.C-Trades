//! Holding the price line open, and everything that can end it.
//!
//! ```text
//!     closed.rs     why the line stopped — neither reason is a fault
//!     refusals.rs   which pairs IBKR refused, and when that is fatal
//!     listen.rs     the loop: prices, and the ten-minute housekeeping tick
//! ```

mod closed;
mod listen;
mod refusals;

#[cfg(test)]
mod tests;

pub use closed::Closed;
pub use listen::listen;
