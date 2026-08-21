//! The raw price stream, kept as proof.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin listen            EUR/USD
//!     cargo run -p nsc-work-man --bin listen -- XAU/USD
//! ```
//!
//! **Every tick printed whole, on purpose.** This is the window onto what IBKR
//! actually sends for a pair — bids and asks arriving separately, the notices
//! in between, and whether anything arrives at all.
//!
//! **TWS or IB Gateway must be running and logged in.**
//!
//! Watch for a `Notice` and no prices. That is IBKR refusing the pair while
//! leaving the line open, and it is the failure that otherwise looks exactly
//! like a quiet market. Gold is the one to check: spot metals are a different
//! market data subscription from spot forex.

use anyhow::Result;
use nsc_data::sources::ibkr::IbkrConnection;

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let symbol = std::env::args().nth(1).unwrap_or_else(|| "EUR/USD".into());

    let ibkr = IbkrConnection::connect().await?;

    ibkr.watch_ticks(&symbol).await?;

    Ok(())
}
