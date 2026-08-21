//! Subscribing to every pair, and folding them into one line.

use ibapi::Client;
use ibapi::contracts::tick_types::TickType;
use ibapi::market_data::realtime::TickTypes;
use ibapi::messages::Notice;
use tokio::sync::mpsc::{Sender, channel};

use crate::source::{Heard, Price, Prices};

use super::super::contract;
use super::super::error::IbkrError;
use super::spread::Spread;

/// How many prices may queue up before the sender waits.
///
/// Prices arrive several times a second across four pairs. The watcher reads
/// them faster than that unless it is drawing a card, and drawing takes
/// seconds — so the room is for those seconds, not for a backlog.
const ROOM: usize = 1024;

/// **One subscription per pair, one line out.**
///
/// Twelve Data took every symbol on a single socket. IBKR does not — each
/// contract is its own subscription — so they are folded back into one channel
/// here. The watcher keeps the loop it always had, and nothing above this file
/// learns that four connections became one.
pub(crate) async fn open(client: &Client, symbols: &[String]) -> Result<Prices, IbkrError> {
    let (send, receive) = channel(ROOM);
    let mut carrying = Vec::new();
    let mut refused = Vec::new();

    for symbol in symbols {
        let contract = contract::for_symbol(symbol)?;

        let subscription = match client.market_data(&contract).streaming().subscribe().await {
            Ok(subscription) => subscription,
            Err(trouble) => {
                refused.push(format!("{symbol} ({trouble})"));
                continue;
            }
        };

        carrying.push(tokio::spawn(carry(
            symbol.clone(),
            subscription,
            send.clone(),
        )));
    }

    // **Asking about nothing is not the same as being refused everything.**
    // Nought equals nought, so an empty list used to read as a total refusal
    // and reported the line as broken over a bot doing what it was told.
    if !symbols.is_empty() && refused.len() == symbols.len() {
        return Err(IbkrError::Refused {
            symbol: symbols.join(", "),
            why: format!("every pair was refused: {}", refused.join("; ")),
        });
    }

    if !refused.is_empty() {
        eprintln!(
            "IBKR would not open a price line for: {}. Watching the other {}.",
            refused.join("; "),
            symbols.len() - refused.len(),
        );
    }

    Ok(Prices::new(receive, carrying))
}

/// Everything one pair says, until it stops saying it.
///
/// **Falling out of the loop is not success.** A line that ends is a line that
/// ended, and on a running market that is never normal — it has to reach the
/// watcher as trouble, or a dead subscription looks exactly like a pair that
/// nothing is happening to.
async fn carry(
    symbol: String,
    mut subscription: ibapi::subscriptions::Subscription<TickTypes>,
    send: Sender<Heard>,
) {
    let mut spread = Spread::default();
    let mut said_delayed = false;

    while let Some(tick) = subscription.next().await {
        let heard = match tick {
            Ok(tick) => read(&symbol, tick, &mut spread, &mut said_delayed),
            Err(trouble) => Some(Heard::Broke {
                symbol: symbol.clone(),
                why: trouble.to_string(),
            }),
        };

        let Some(heard) = heard else { continue };

        // The other end has gone. Nothing to carry prices to.
        if send.send(heard).await.is_err() {
            return;
        }
    }

    let _ = send
        .send(Heard::Broke {
            symbol,
            why: "the price line closed on its own".into(),
        })
        .await;
}

/// One tick, as one thing the watcher understands — or as nothing at all.
pub(super) fn read(
    symbol: &str,
    tick: TickTypes,
    spread: &mut Spread,
    said_delayed: &mut bool,
) -> Option<Heard> {
    let (which, raw) = match tick {
        TickTypes::Price(one) => (one.tick_type, one.price),
        TickTypes::PriceSize(one) => (one.price_tick_type, one.price),
        TickTypes::Notice(notice) => return from_notice(symbol, notice),

        // Sizes, option maths, exchange-for-physical. None of it is a price.
        _ => return None,
    };

    // **Delayed prices are said out loud, once, and then ignored.**
    //
    // An account without live forex data is served fifteen-minute-old prices
    // instead of nothing. Dropped quietly, the bot sits silent and looks
    // exactly like a market where nothing is happening. Acted on, it tells him
    // price is at his level a quarter of an hour after it was.
    if matches!(which, TickType::DelayedBid | TickType::DelayedAsk) {
        if std::mem::replace(said_delayed, true) {
            return None;
        }

        return Some(Heard::Refused {
            symbol: symbol.to_string(),
            why: "IBKR is sending DELAYED prices — this account has no live data for it".into(),
        });
    }

    let mid = spread.took(which, raw)?;

    Some(Heard::Price(Price {
        symbol: symbol.to_string(),
        mid,
    }))
}

/// What TWS said, and whether it is worth passing on.
///
/// **Most notices are IBKR clearing its throat.** "Market data farm connection
/// is OK" arrives on every connection. Passing those on as refusals would
/// report a healthy feed as broken every time it started.
pub(super) fn from_notice(symbol: &str, notice: Notice) -> Option<Heard> {
    match notice.code {
        // The data farms saying hello, and connectivity coming back.
        2100..=2200 | 1101 | 1102 => None,

        // The line between IB and TWS went down. TWS is still there; the data
        // behind it is not.
        1100 => Some(Heard::Broke {
            symbol: symbol.to_string(),
            why: notice.message,
        }),

        // Everything else is IBKR saying no — usually a market data
        // subscription the account does not have.
        _ => Some(Heard::Refused {
            symbol: symbol.to_string(),
            why: format!("{} ({})", notice.message, notice.code),
        }),
    }
}
