//! One live price, and what it is the middle of.

use rust_decimal::Decimal;

/// A price as the bot understands it.
///
/// **It is the middle of the spread, never one side of it.** The candles come
/// back as `MidPoint` — see `sources/ibkr/candles.rs` — so a live price taken
/// from the bid would be measured against bands drawn on mid prices. On the
/// euro that is a fifth of a pip and nobody notices. On gold the spread is
/// around 30 cents, which is most of a band edge: the alert says price
/// touched the level and the candle card then says it never got there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Price {
    /// The pair, spelled the way `config/pairs` spells it — `EUR/USD`.
    pub symbol: String,

    /// Halfway between the bid and the ask.
    pub mid: Decimal,
}

/// What comes down the price line.
///
/// **Three things, not one, and the difference is the whole point.**
///
/// ```text
///     Price     a price arrived                    the normal case
///     Refused   IBKR will not serve this pair      SILENT otherwise
///     Broke     this pair's line ended             open it again
/// ```
///
/// `Refused` exists because of how IBKR says no. It does not fail the
/// subscription — it sends a notice down the open line and then never sends a
/// price. Without this variant that is indistinguishable from a quiet market,
/// which is the failure this project keeps having: nothing arrives, nothing
/// errors, and the bot looks like it is working.
#[derive(Debug, Clone)]
pub enum Heard {
    /// A price, in the middle of the spread.
    Price(Price),

    /// IBKR answered, and said no. Usually a market data subscription the
    /// account does not have — spot metals are a separate one from spot forex.
    Refused { symbol: String, why: String },

    /// This pair's line ended. The others may still be running.
    Broke { symbol: String, why: String },
}

/// The open price line, and the tasks feeding it.
///
/// **Dropping this stops the subscriptions.** That is the whole reason it is a
/// type rather than a bare channel.
///
/// Each pair is carried by its own task sitting on `subscription.next()`. A
/// task only notices the line has been put away when it next tries to send —
/// so a quiet pair would sit there **forever**, holding an IBKR market data
/// line that nobody is reading.
///
/// The watcher reopens the line every time he sends a level. Twenty levels
/// over a weekend is twenty abandoned subscriptions per pair, against an IBKR
/// limit that is counted in lines, not in pairs. It would stop serving new
/// ones and nothing would say why.
pub struct Prices {
    line: tokio::sync::mpsc::Receiver<Heard>,
    carrying: Vec<tokio::task::JoinHandle<()>>,
}

impl Prices {
    pub(crate) fn new(
        line: tokio::sync::mpsc::Receiver<Heard>,
        carrying: Vec<tokio::task::JoinHandle<()>>,
    ) -> Self {
        Prices { line, carrying }
    }

    /// The next thing off the line, or `None` once every pair has stopped.
    pub async fn next(&mut self) -> Option<Heard> {
        self.line.recv().await
    }

    /// How many pairs are actually being listened to.
    ///
    /// **Not how many were asked for.** A pair IBKR would not subscribe to at
    /// all never gets a task, so counting the asking would leave the caller
    /// waiting for a refusal that can never arrive.
    pub fn watching(&self) -> usize {
        self.carrying.len()
    }
}

impl Drop for Prices {
    fn drop(&mut self) {
        for one in &self.carrying {
            one.abort();
        }
    }
}
