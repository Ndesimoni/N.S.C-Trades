//! Finding holes in the data before they poison the analysis.
//!
//! Scans saved candles for missing ones and works out whether each gap is
//! expected (weekend, holiday, genuinely dead session) or unexplained.
//!
//! This exists because bad data does not fail loudly. A missing hour shifts a
//! swing high, which shifts a level, which changes every signal after it — and
//! the backtest still finishes and prints a perfectly believable number.
//!
//! Run this after every download, and refuse to trust results from history you
//! have not checked.
