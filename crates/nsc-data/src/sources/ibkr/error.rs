//! What can go wrong talking to IBKR, and what to do about each one.
//!
//! Four kinds, because they need four different answers:
//!
//! ```text
//!     Setup      a missing .env line          GIVE UP — retrying never adds it
//!     Connection cannot reach the gateway     RETRY  — it may come back
//!     Stream     the price line died          RETRY  — subscribe again
//!     Refused    IBKR will not serve the pair GIVE UP — a subscription is missing
//! ```
//!
//! Lumping these into one is the mistake this file exists to prevent. A
//! missing `.env` line and a dropped connection look identical from outside,
//! so a bot that cannot tell them apart retries the missing line forever — and
//! on screen that is indistinguishable from a gateway that is down.

use std::error::Error;

use nsc_core::error::{Answer, Knows};
use thiserror::Error as Thiserror;

/// A boxed cause. IBKR, the environment and the number parser all return
/// different error types and none of them are ours to name.
pub type Cause = Box<dyn Error + Send + Sync>;

#[derive(Debug, Thiserror)]
pub enum IbkrError {
    /// Something is missing or unreadable in `.env`. **Give up.**
    ///
    /// Carries the setting's name as text rather than the original error,
    /// because `VarError` only ever says "environment variable not found" and
    /// there are three of them to choose from.
    #[error("IBKR setup: {0}. Check .env in the project root.")]
    Setup(String),

    /// Could not reach the gateway. **Retry.**
    ///
    /// Nearly always means TWS or IB Gateway is not running. Unlike a cloud
    /// API, this feed needs a desktop program logged in somewhere reachable.
    #[error("could not reach IBKR — is TWS or IB Gateway running and logged in?")]
    Connection(#[source] Cause),

    /// The price line broke while we were listening. **Subscribe again.**
    ///
    /// Separate from `Connection` because the gateway is still there. Only
    /// this one subscription died, and the others are still running.
    #[error("the IBKR price line broke")]
    Stream(#[source] Cause),

    /// IBKR answered, and said no. **Give up.**
    ///
    /// Usually a market data subscription the account does not have. Gold is
    /// the one that bites: spot metals are a separate subscription from spot
    /// forex, and a paper account often has neither.
    #[error("IBKR will not serve {symbol}: {why}")]
    Refused { symbol: String, why: String },

    /// A candle came back that could not be turned into one of ours.
    ///
    /// **Give up and shout.** Guessing at a candle corrupts history, and bad
    /// history does not fail loudly — it shifts a swing, which shifts a level,
    /// which changes every signal after it.
    #[error("IBKR sent a candle that could not be read: {0}")]
    NotACandle(String),
}

impl Knows for IbkrError {
    fn answer(&self) -> Answer {
        match self {
            // No amount of waiting puts a line in a file.
            IbkrError::Setup(_) => Answer::GiveUp,

            // The gateway may be starting up, or the machine may have slept.
            IbkrError::Connection(_) => Answer::soon(),

            // One subscription died. The gateway is fine.
            IbkrError::Stream(_) => Answer::soon(),

            // A missing subscription is missing until somebody buys it.
            // Retrying looks exactly like a dead feed and wastes an evening.
            IbkrError::Refused { .. } => Answer::GiveUp,

            // Their shape changed, or ours is wrong. Either way, asking again
            // gets the same unreadable answer.
            IbkrError::NotACandle(_) => Answer::GiveUp,
        }
    }
}
