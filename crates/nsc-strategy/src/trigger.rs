//! Layer 3 — the thing that actually starts the trade.
//!
//! Direction says which way. Place says where. This says **when**.
//!
//! It is the layer most traders have thought about least, and it deserves the
//! most attention, because it sets your entry price and therefore the
//! risk-to-reward of every trade the system ever takes.
//!
//! Kinds of trigger: a candlestick pattern at the zone, a break of some small
//! structure, or price closing back inside a level after poking through it.
//!
//! Two guards worth keeping. The trigger candle must close in the direction
//! you are trading. And an oversized trigger candle gets rejected — by the
//! time a huge candle has closed, the move you wanted is mostly gone and your
//! stop has to sit uselessly far away.
