//! Layer 4 — where the stop goes, and why there.
//!
//! Options: past the swing that made the level, a multiple of normal candle
//! size, or past the trigger candle's wick.
//!
//! The **rule** matters more than the number, because it has to work on every
//! pair and in every kind of market. "Past the swing plus a bit" travels.
//! "35 pips" does not.
//!
//! If the resulting stop is wider than the maximum, the setup is dropped
//! rather than squeezed to fit. Shrinking a stop to make a trade work is how a
//! system starts taking trades whose real invalidation sits far beyond where
//! the money is now at risk.
