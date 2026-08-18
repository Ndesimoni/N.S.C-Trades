use std::env;

use ibapi::{Client, register_timezone_alias};

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

        Ok(Self { client })
    }
}
