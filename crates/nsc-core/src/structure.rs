//! Trend direction, read from the sequence of swings.
//!
//! Higher highs and higher lows means uptrend. Lower highs and lower lows
//! means downtrend. This module also flags the two events that change it:
//!
//!   - **break of structure** — price makes a new extreme, trend continues
//!   - **change of character** — the first swing that breaks the pattern, the
//!     earliest warning of a reversal
//!
//! This is the default way the bot decides which direction it is allowed to
//! trade. Without it, the bot will happily send you a buy and a sell on the
//! same pair in the same session, and you will stop trusting it.
