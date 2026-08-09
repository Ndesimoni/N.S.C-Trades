//! Drawing the candles.
//!
//! Draws a window of candles around the signal with sensible spacing, sized
//! for a phone rather than a monitor — that is where these actually get read.
//!
//! Shows a fixed number of candles before the trigger on purpose, so every
//! chart you look at is framed the same way. Consistent framing is what lets
//! you judge quickly. A window that changes size means re-orienting yourself
//! on every image.
