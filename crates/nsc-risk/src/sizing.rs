//! Working out how big the trade should be.
//!
//! From your account size, the percentage you risk, and how far away the stop
//! is. Printed on every signal so you are not doing arithmetic at the exact
//! moment arithmetic tends to go wrong.
//!
//! The fiddly bit is pairs where your account currency is neither of the two
//! currencies involved. Sizing a EURGBP trade on a dollar account needs a
//! conversion rate, and getting it wrong quietly misstates your risk on every
//! cross you trade.
