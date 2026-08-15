//! Listen to Twelve Data's live price stream.
//!
//! A separate program from the bot, because it has a different shape: the bot
//! does one job and exits, this one holds a line open and waits.
//!
//! Right now it asks one question — **will the trial plan give us gold?**
//! Everything we plan to build depends on the answer.

use anyhow::{Context, Result, bail};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

// Bitcoin on purpose, for now. Gold is shut at the weekend and a silent line
// looks exactly like a broken one. Crypto trades all weekend, so it answers
// everything except "does gold specifically tick" — and we already know gold
// is allowed on this plan.
const SYMBOL: &str = "BTC/USD";

/// Stop after this many prices. Enough to see the shape and the rate.
const ENOUGH: usize = 15;

/// If nothing arrives for this long, say so rather than hanging.
const PATIENCE: std::time::Duration = std::time::Duration::from_secs(30);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let key = std::env::var("TWELVE_DATA_API_KEY")
        .context("TWELVE_DATA_API_KEY is not set. Is there a .env file in the project root?")?;

    // `wss` not `https` — a line that stays open. The key rides in the
    // address, so never print this.
    let url = format!("wss://ws.twelvedata.com/v1/quotes/price?apikey={key}");

    println!("Opening the line...");

    let (mut socket, reply) = connect_async(&url)
        .await
        .context("the line would not open")?;

    println!("Open. Twelve Data answered {}\n", reply.status());

    // Opening the line gets us nothing on its own. You have to say what you
    // want to hear about.
    let ask = serde_json::json!({
        "action": "subscribe",
        "params": { "symbols": SYMBOL }
    });

    println!("Asking for {SYMBOL}...");

    socket
        .send(Message::Text(ask.to_string()))
        .await
        .context("could not send the request")?;

    // The first thing back says whether we are allowed.
    let Some(answer) = socket.next().await else {
        bail!("the line closed without answering");
    };

    println!("\n{}\n", answer.context("the reply could not be read")?);
    println!("Listening...\n");

    // Then the prices arrive on their own. Nobody asks for them — that is the
    // whole difference between a line and a request.
    let started = std::time::Instant::now();
    let mut seen = 0;

    while seen < ENOUGH {
        match tokio::time::timeout(PATIENCE, socket.next()).await {
            Err(_) => {
                println!("\nNothing for 30 seconds. Is the market open?");
                break;
            }
            Ok(None) => {
                println!("\nThe other side hung up.");
                break;
            }
            Ok(Some(message)) => {
                println!("{}", message.context("the line broke")?);
                seen += 1;
            }
        }
    }

    println!(
        "\n{seen} prices in {:.1} seconds",
        started.elapsed().as_secs_f64()
    );

    Ok(())
}
