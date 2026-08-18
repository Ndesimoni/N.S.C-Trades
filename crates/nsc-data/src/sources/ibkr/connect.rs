use std::env;

use futures_util::StreamExt;

use ibapi::{
    Client,
    accounts::{AccountSummaryResult, AccountSummaryTags, types::AccountGroup},
    register_timezone_alias,
};

use ibapi::contracts::{Contract, Currency, Exchange, SecurityType, Symbol};

use ibapi::contracts::tick_types::TickType;
use ibapi::market_data::realtime::TickTypes;

use super::error::IbkrError;

pub struct IbkrConnection {
    pub client: Client,
}

impl IbkrConnection {
    pub async fn connect() -> Result<Self, IbkrError> {
        register_timezone_alias("Gulf Standard Time", "Asia/Dubai");

        let host = env::var("IBKR_HOST").map_err(|e| IbkrError::Connection(Box::new(e)))?;

        let port = env::var("IBKR_PORT").map_err(|e| IbkrError::Connection(Box::new(e)))?;

        let client_id: i32 = env::var("IBKR_CLIENT_ID")
            .map_err(|e| IbkrError::Connection(Box::new(e)))?
            .parse()
            .map_err(|e| IbkrError::Connection(Box::new(e)))?;

        let address = format!("{host}:{port}");

        println!("Connecting to IBKR at {address} with client ID {client_id}...");

        let client = Client::connect(&address, client_id)
            .await
            .map_err(|e| IbkrError::Connection(Box::new(e)))?;

        println!("Connected to IBKR.");

        let contract = Contract {
            symbol: Symbol::from("EUR"),
            security_type: SecurityType::ForexPair,
            exchange: Exchange::from("IDEALPRO"),
            currency: Currency::from("USD"),
            ..Contract::default()
        };

        let mut subscription = self
            .client
            .market_data(&contract, &[], false, false)
            .await
            .map_err(|e| IbkrError::Connection(Box::new(e)))?;

        println!("========== EUR/USD LIVE ==========");

        while let Some(tick) = subscription.next().await {
            match tick {
                Ok(TickTypes::Price(price)) => match price.tick_type {
                    TickType::Bid => println!("EUR/USD BID: {}", price.price),
                    TickType::Ask => println!("EUR/USD ASK: {}", price.price),
                    _ => {}
                },
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Market data error: {e}");
                    break;
                }
            }
        }

        Ok(Self { client })
    }

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
                AccountSummaryResult::Summary(summary) => {
                    println!("{summary:?}");
                }

                AccountSummaryResult::End => {
                    println!("==================================");
                    break;
                }
            }
        }

        Ok(())
    }
}
