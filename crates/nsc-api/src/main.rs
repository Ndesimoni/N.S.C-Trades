//! # nsc-api — the web endpoints
//!
//! Two jobs, sitting behind Nginx:
//!
//!   1. Receive Telegram button presses — the 👍/👎 that build your dataset
//!   2. Serve the admin pages — signal history, stats, backtest triggers, and
//!      the chart replay tool for labelling old setups
//!
//! Kept as a separate process from `nsc-live` on purpose. This one takes
//! input from the open internet. The bot that reads charts should not share a
//! process with it, and restarting the website should never interrupt the
//! price feed.

mod error;
mod routes;
mod state;

fn main() {
    // Phase 0: start axum, mount the routes, share the database pool.
    todo!("see routes/ for the intended endpoints")
}
