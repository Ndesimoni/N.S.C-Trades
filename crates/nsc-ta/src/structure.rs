//! Working out the trend, and spotting when it changes.
//!
//! Walks through the confirmed swings and decides: higher highs and higher
//! lows (uptrend), or lower highs and lower lows (downtrend). Then flags:
//!
//!   - **break of structure** — a new extreme, the trend carrying on
//!   - **change of character** — the first swing that breaks the pattern, the
//!     earliest hint of a reversal
//!
//! How far price must push through to count as a break is measured against
//! normal candle size, so a one-pip poke past an old high does not count.
//!
//! This is the default way the bot picks a direction. It is the cheapest way
//! to stop it sending you a buy and a sell on the same pair on the same day.
