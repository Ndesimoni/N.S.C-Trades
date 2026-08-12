//! The actual broker connections. Picked at startup by an environment
//! variable.
//!
//! What you are choosing between, since it shapes your whole setup:
//!
//!   `oanda`      Works on macOS and Linux with no extra machinery, free with
//!                an account. The path of least resistance.
//!
//!   `mt5_bridge` What most retail forex traders already use — but MetaTrader
//!                5's programming interface only runs on Windows. So this
//!                talks to a script inside the terminal over a socket instead.
//!                That means a Windows server and a second program to babysit.
//!
//!   `twelvedata` A clean data vendor. Works on macOS, costs money, and is
//!                separate from wherever you eventually place trades.
//!
//!   `csv`        Replaying a file. Not a fallback — the fastest way to build
//!                and test, because it needs no internet and never changes
//!                underneath you.

pub mod csv_source;
pub mod mt5_bridge;
pub mod oanda;
pub mod twelvedata;

pub use csv_source::read_candles;
