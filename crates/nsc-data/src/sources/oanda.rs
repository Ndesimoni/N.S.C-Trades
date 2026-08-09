//! OANDA connection.
//!
//! Practical notes: OANDA names timeframes slightly differently and the names
//! get mapped here. History comes back in pages with a cap per request, so
//! downloading works in chunks. When the daily candle closes is a request
//! setting — set it to match `config/app.toml` instead of taking the default.
//!
//! OANDA gives bid, ask and mid separately. Store mid for reading the chart
//! and keep the spread for the skip check. A setup that looks fine on mid
//! prices and loses money after a 4-pip spread is not a setup.
