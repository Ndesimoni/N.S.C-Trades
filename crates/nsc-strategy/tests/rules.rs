//! Tests for the rules, using made-up snapshots.
//!
//! This is the payoff for the crate never touching the outside world. A
//! snapshot takes a few lines to build by hand, so each of the six layers can
//! be tested on its own — no database, no broker, not a single real candle.
//!
//! Worth having from day one:
//!   - each must-pass rule can reject on its own, and which layer rejected
//!     gets recorded
//!   - counter-trend setups get refused when they are switched off
//!   - a stop that comes out too wide kills the setup instead of being
//!     squeezed to fit
//!   - all three target methods on one identical setup, so the difference in
//!     risk-to-reward is visible rather than assumed
//!   - adding a confluence never lowers the score
//!   - a nonsensical settings file fails when it LOADS, not on the first quiet
//!     week
//!
//! That last one saves the most time. A settings file asking for more
//! confluences than it has sources switched on shows up as "the bot never
//! sends anything", which is miserable to work out from the outside.
