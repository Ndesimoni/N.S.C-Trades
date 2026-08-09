//! Shared things the web endpoints need: database pool, Redis, settings.
//!
//! Mostly read-only. The web layer reads history and queues work. It never
//! applies your trading rules.
//!
//! Rules are applied in exactly one place — `nsc-strategy`, run by the live
//! bot or the backtester. A second place that applied rules would be the first
//! crack in the promise that backtest and live agree.
