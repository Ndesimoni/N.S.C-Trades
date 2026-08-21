//! Opening the line to TWS, and what can be asked of it once it is open.

use std::env;

use ibapi::{
    Client,
    accounts::{AccountSummaryResult, AccountSummaryTags, types::AccountGroup},
    register_timezone_alias,
};

use crate::source::Prices;

use super::contract;
use super::error::IbkrError;
use super::serves::{Serves, never_heard_of_it};
use super::ticks;

pub struct IbkrConnection {
    pub(super) client: Client,
}

impl IbkrConnection {
    /// Open the line to TWS or IB Gateway.
    ///
    /// **The timezone alias is not optional and must come first.** TWS reports
    /// the machine's timezone using a Windows name — "Gulf Standard Time" —
    /// and the library only knows the IANA ones. Without the alias, connecting
    /// fails outright with an error that says nothing about timezones.
    pub async fn connect() -> Result<Self, IbkrError> {
        Self::open(0).await
    }

    /// A second line to TWS, one client id along.
    ///
    /// **IBKR allows one connection per client id**, and a second connection
    /// on the same id throws the first one off. The watcher holds the id from
    /// `.env` for weeks at a time; anything else that needs candles — the
    /// inbox drawing him a chart — has to come in on its own.
    pub async fn connect_beside() -> Result<Self, IbkrError> {
        Self::open(1).await
    }

    /// The line itself. `along` is added to the client id from `.env`.
    async fn open(along: i32) -> Result<Self, IbkrError> {
        register_timezone_alias("Gulf Standard Time", "Asia/Dubai");

        let host =
            env::var("IBKR_HOST").map_err(|_| IbkrError::Setup("IBKR_HOST is not set".into()))?;

        let port =
            env::var("IBKR_PORT").map_err(|_| IbkrError::Setup("IBKR_PORT is not set".into()))?;

        let client_id: i32 = env::var("IBKR_CLIENT_ID")
            .map_err(|_| IbkrError::Setup("IBKR_CLIENT_ID is not set".into()))?
            .parse()
            .map_err(|_| IbkrError::Setup("IBKR_CLIENT_ID is not a whole number".into()))?;

        let client_id = client_id + along;
        let address = format!("{host}:{port}");

        println!("Connecting to IBKR at {address} with client ID {client_id}...");

        let client = Client::connect(&address, client_id)
            .await
            .map_err(|e| IbkrError::Connection(Box::new(e)))?;

        println!("Connected to IBKR.");

        Ok(Self { client })
    }

    /// The live price line for every pair being watched.
    ///
    /// **One subscription per pair, folded into one channel.** Twelve Data
    /// carried every symbol on a single socket; IBKR gives one connection per
    /// contract. The folding happens in `ticks/`, so the watcher keeps the one
    /// loop it has always had.
    ///
    /// What comes down it is `Heard` — a price, a refusal, or a line that
    /// ended. **A refusal has to travel**, because IBKR does not fail a
    /// subscription it will not serve: it sends one notice down an open line
    /// and then stays silent forever, which is indistinguishable from a quiet
    /// market.
    pub async fn prices(&self, symbols: &[String]) -> Result<Prices, IbkrError> {
        ticks::open(&self.client, symbols).await
    }

    /// Print every tick for a pair until the line breaks.
    ///
    /// **It does not return while the market is running.** That is the point —
    /// it is a window onto the feed, not a step in a sequence.
    ///
    /// Every tick is printed whole, on purpose. What IBKR sends for a forex
    /// pair is still an open question, and a match written before seeing the
    /// answer would filter out the answer.
    pub async fn watch_ticks(&self, symbol: &str) -> Result<(), IbkrError> {
        let contract = contract::for_symbol(symbol)?;

        let mut subscription = self
            .client
            .market_data(&contract)
            .streaming()
            .subscribe()
            .await
            .map_err(|e| IbkrError::Refused {
                symbol: symbol.to_string(),
                why: e.to_string(),
            })?;

        println!("========== {symbol} LIVE ==========");

        while let Some(tick) = subscription.next().await {
            let tick = tick.map_err(|e| IbkrError::Stream(Box::new(e)))?;
            println!("{symbol} TICK: {tick:?}");
        }

        // **Getting here means the other side hung up.** On a live feed that is
        // never normal, and it must not read as success — a feed that stopped
        // looks exactly like a market where nothing happened.
        Err(IbkrError::Stream("the price line closed on its own".into()))
    }

    /// **Does IBKR have any idea what this instrument is?**
    ///
    /// The broker is the authority, and nothing else is. A spelling check
    /// catches `AUDUS` and `AUDUSDD`; it does NOT catch `AUDUSS`, which is the
    /// right shape with a currency that does not exist — and that is exactly
    /// the typo a thumb makes.
    ///
    /// **Three answers, and the third is why this is not a `bool`.** `Never`
    /// means IBKR looked and found nothing, which is grounds for refusing a
    /// pair. An `Err` means it could not be asked — TWS shut, the line down —
    /// and that must NEVER read as "no such pair", or one gateway outage would
    /// retire every pair he owns.
    pub async fn serves(&self, symbol: &str) -> Result<Serves, IbkrError> {
        let contract = match contract::for_symbol(symbol) {
            Ok(contract) => contract,

            // It could not even be written as an instrument. That is a
            // definite no, and it costs nothing to say so without asking.
            Err(IbkrError::Refused { why, .. }) => return Ok(Serves::Never { why }),
            Err(other) => return Err(other),
        };

        match self.client.contract_details(&contract).await {
            Ok(found) if found.is_empty() => Ok(Serves::Never {
                why: "IBKR has no instrument by that name".into(),
            }),
            Ok(_) => Ok(Serves::Yes),

            // IBKR looked and found nothing.
            Err(ibapi::Error::Message(code, said)) if never_heard_of_it(code) => {
                Ok(Serves::Never { why: said })
            }

            // Anything else is us failing to ASK, not IBKR saying no.
            Err(trouble) => Err(IbkrError::Connection(Box::new(trouble))),
        }
    }

    /// What the account holds, printed once.
    pub async fn account_summary(&self) -> Result<(), IbkrError> {
        let group = AccountGroup("All".to_string());

        let mut subscription = self
            .client
            .account_summary(&group, AccountSummaryTags::ALL)
            .await
            .map_err(|e| IbkrError::Connection(Box::new(e)))?;

        println!("========== IBKR ACCOUNT ==========");

        while let Some(result) = subscription.next().await {
            let result = result.map_err(|e| IbkrError::Connection(Box::new(e)))?;

            match result {
                AccountSummaryResult::Summary(summary) => println!("{summary:?}"),
                AccountSummaryResult::End => {
                    println!("==================================");
                    break;
                }
            }
        }

        Ok(())
    }
}
