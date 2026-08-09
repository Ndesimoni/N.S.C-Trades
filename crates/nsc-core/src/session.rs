//! Trading sessions — Sydney, Tokyo, London, New York, and their overlaps.
//!
//! Sessions matter twice over. They are something to skip on: many setups are
//! only worth taking during London or the London/New York overlap. And they
//! are useful information: the same candle pattern at the Tokyo open and at
//! the London open is not the same trade.
//!
//! All the boundaries are worked out in UTC from the configured local times,
//! so daylight saving is handled once here instead of being wrong in several
//! places.
