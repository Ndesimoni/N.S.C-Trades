//! Draw a pair's levels and send the picture.
//!
//!     cargo run -p nsc-work-man --bin levels -- GBPUSD
//!
//! The same picture `inbox` sends after saving. This one is for looking at a
//! pair without sending anything.

use std::path::Path;

use anyhow::{Context, Result};
use nsc_core::levels::{load_pair, load_thickness};
use nsc_work_man::{review, telegram};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();

    let wanted = std::env::args().nth(1).unwrap_or_else(|| "XAUUSD".into());
    let file = Path::new("config/pairs").join(format!("{wanted}.toml"));

    let pair = load_pair(&file)
        .with_context(|| format!("no levels for {wanted} — is there a {}?", file.display()))?;

    let thickness = load_thickness(Path::new("config/levels.toml"))?;

    for line in &pair.levels {
        println!("{:7} {}", line.timeframe.name(), line.price);
    }

    let picture = Path::new("preview").join("levels.png");
    review::picture_of(&client, &pair, thickness, &picture).await?;

    let caption = format!(
        "<b>{}</b> · weekly · {} level{} you drew",
        pair.symbol,
        pair.levels.len(),
        if pair.levels.len() == 1 { "" } else { "s" }
    );

    telegram::send(&client, &[&picture], &caption).await?;
    println!("\nsent to your channel.");

    Ok(())
}
