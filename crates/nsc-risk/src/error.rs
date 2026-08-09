//! Things that go wrong working out risk.
//!
//! Note what is **not** an error here. A setup blocked by an exposure limit or
//! a tripped brake is a normal, expected outcome. It comes back as a decision,
//! not a failure. Blocks are the system working.
//!
//! Real errors are narrow — mainly a missing conversion rate when sizing a
//! cross on an account in a third currency. That one matters, because the
//! tempting shortcut is to assume a rate of 1.0, which silently misstates your
//! risk on every cross you trade.
