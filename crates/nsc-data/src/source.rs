//! `MarketDataSource` — the trait every broker hides behind.
//!
//! Two jobs: fetch old candles for a date range, and subscribe to new ones.
//!
//! Each implementation smooths out its provider's quirks — timestamp format
//! and timezone, whether prices are bid or mid, weekend gaps, and the
//! provider's own idea of when a day ends — so nothing downstream ever learns
//! which broker it is talking to.
//!
//! What every implementation must guarantee:
//!   - timestamps in UTC, marking when the candle **started**
//!   - unfinished candles flagged, never quietly included
//!   - mid prices, unless the provider only gives bid
