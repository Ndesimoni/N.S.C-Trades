//! Draw a pair's levels and send the picture.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin levels -- GBPUSD
//! ```
//!
//! The same picture `inbox` sends after saving. This one is for looking at a
//! pair without sending anything.

use std::path::Path;

use anyhow::{Context, Result};
use nsc_core::levels::{Timeframe, load_pair, load_thickness};
use nsc_data::sources::ibkr::IbkrConnection;
use nsc_work_man::places::{PAIRS, PREVIEW, THICKNESS};
use nsc_work_man::{review, telegram};

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let client = reqwest::Client::new();

    // TWS or IB Gateway has to be running. Every candle comes from it.
    let ibkr = IbkrConnection::connect().await?;

    let wanted = std::env::args().nth(1).unwrap_or_else(|| "XAUUSD".into());
    let file = Path::new(PAIRS).join(format!("{wanted}.toml"));

    let pair = load_pair(&file)
        .with_context(|| format!("no levels for {wanted} — is there a {}?", file.display()))?;

    let thickness = load_thickness(Path::new(THICKNESS))?;

    for line in &pair.levels {
        println!("{:7} {}", line.timeframe.name(), line.price);
    }

    // The weekly, because this prints every level a pair has and only the
    // weekly is wide enough to hold levels drawn years apart.
    let out = Path::new(PREVIEW).join("levels.png");
    let drawn = review::picture_of(&ibkr, &pair, thickness, Timeframe::Weekly, &out).await?;

    let caption = format!(
        "📐 <b>{}</b> — the {} level{} you drew, on the weekly chart.",
        pair.symbol,
        pair.levels.len(),
        if pair.levels.len() == 1 { "" } else { "s" }
    );

    telegram::send(&client, &[&drawn.picture], &caption).await?;
    println!("\nSent to your channel.");

    Ok(())
}
