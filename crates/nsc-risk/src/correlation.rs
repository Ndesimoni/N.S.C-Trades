//! Stopping you from taking the same bet five times.
//!
//! Long EURUSD, short USDJPY, long AUDUSD and long NZDUSD is not four trades.
//! It is one bet against the dollar at four times the size you intended.
//!
//! That is how a disciplined 1%-per-trade rule turns into a 4% loss on a
//! single dollar headline.
//!
//! The groups are listed in `config/symbols.toml` rather than calculated from
//! rolling correlation. Fixed groups are predictable and you can see why they
//! blocked something. Rolling correlation shifts underneath you and produces
//! blocks you cannot explain afterwards.
