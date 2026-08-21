//! The bot.
//!
//! ```text
//!     cargo run -p nsc-work-man
//! ```
//!
//! **It sends signals and places no trades.** Version 1 has no execution in
//! it, and `features.execution` being absent is not a gap waiting to be
//! helpfully filled in.
//!
//! **TWS or IB Gateway has to be running and logged in.** Every candle and
//! every price comes from IBKR, so without it there is no feed at all. The bot
//! says so and stops rather than starting up watching nothing.
//!
//! Everything it does is in `nsc_work_man::watch`. This file exists so that
//! the obvious command runs the real thing — it used to run a leftover from
//! step one that sent a gold card every time it was called, which is the exact
//! opposite of the rule the whole design rests on: SILENCE IS THE DEFAULT.

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let Err(trouble) = nsc_work_man::watch::run().await else {
        return Ok(());
    };

    // **It only gets here for trouble it cannot recover from** — a gateway
    // that will not answer at startup, a config file that will not parse. The
    // line dropping is handled inside and does not reach this.
    //
    // He has to be told, because from his side a bot that stopped looks
    // exactly like a market where nothing happened.
    eprintln!("Stopping: {trouble:#}");
    nsc_work_man::watch::dying(&reqwest::Client::new(), &format!("{trouble:#}")).await;

    Err(trouble)
}
