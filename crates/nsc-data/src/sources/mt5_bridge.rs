//! MetaTrader 5, through a socket.
//!
//! MT5's programming interface only runs on Windows, so this does not talk to
//! MT5 directly. A script running inside the terminal pushes candles over a
//! socket and this reads them.
//!
//! Be clear-eyed about what choosing this costs:
//!   - a Windows server, plus a second program to keep an eye on
//!   - the terminal has to stay logged in; if the session drops, so does your
//!     data
//!   - broker server time is usually **not** UTC and differs between brokers,
//!     so the offset has to be configured, never assumed
//!
//! Worth it only if you specifically need your own broker's exact prices.
