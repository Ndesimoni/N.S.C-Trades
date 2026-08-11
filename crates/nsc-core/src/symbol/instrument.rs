//! One tradeable instrument.

use std::fmt;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::class::AssetClass;
use super::currency::Currency;
use crate::error::CoreError;
use crate::price::{Pips, Price, PriceDistance};

/// One tradeable instrument and everything the system needs to know about it.
///
/// Built from `config/symbols.toml`. Deliberately broker-neutral — nothing in
/// here says which broker you use, so swapping brokers touches one file
/// elsewhere and none of this.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Symbol {
    name: String,
    class: AssetClass,
    pip_size: Decimal,
    digits: u32,
    max_spread: Pips,
    base: Option<Currency>,
    quote: Option<Currency>,
}

impl Symbol {
    /// Rejects settings that would make every later calculation meaningless.
    ///
    /// A pip size of zero would make every stop distance a divide by zero.
    /// Better to refuse at startup, with the instrument named, than to fail
    /// on a candle six hours into a backtest where you would have no idea
    /// which instrument caused it.
    pub fn new(
        name: &str,
        class: AssetClass,
        pip_size: Decimal,
        digits: u32,
        max_spread: Pips,
        base: Option<Currency>,
        quote: Option<Currency>,
    ) -> Result<Self, CoreError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(CoreError::EmptySymbolName);
        }

        if pip_size <= Decimal::ZERO {
            return Err(CoreError::InvalidPipSize {
                symbol: name.to_string(),
                pip_size: pip_size.to_string(),
            });
        }

        Ok(Self {
            name: name.to_string(),
            class,
            pip_size,
            digits,
            max_spread,
            base,
            quote,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn class(&self) -> AssetClass {
        self.class
    }

    pub fn pip_size(&self) -> Decimal {
        self.pip_size
    }

    pub fn digits(&self) -> u32 {
        self.digits
    }

    pub fn max_spread(&self) -> Pips {
        self.max_spread
    }

    pub fn base(&self) -> Option<&Currency> {
        self.base.as_ref()
    }

    pub fn quote(&self) -> Option<&Currency> {
        self.quote.as_ref()
    }

    /// Does this instrument have anything to do with the given currency?
    ///
    /// The news filter's question. A US jobs number moves EURUSD and USDJPY;
    /// it does not move GBPAUD. Instruments with no currencies at all, like
    /// US30, answer `false` — they need their own rule, not a currency one.
    pub fn involves(&self, currency: &Currency) -> bool {
        self.base.as_ref() == Some(currency) || self.quote.as_ref() == Some(currency)
    }

    /// Turns a distance into pips for this instrument.
    ///
    /// A convenience that saves every caller remembering to fetch `pip_size`
    /// first — the sort of step that gets forgotten once and then produces a
    /// stop ten times too wide.
    pub fn to_pips(&self, distance: PriceDistance) -> Result<Pips, CoreError> {
        distance.to_pips(self.pip_size)
    }

    /// Is the live spread tight enough to trade?
    ///
    /// A skip rule. However good a setup looks, a spread wider than the
    /// instrument's limit eats the edge before the trade starts.
    pub fn spread_is_acceptable(&self, spread: Pips) -> bool {
        spread <= self.max_spread
    }

    /// Rounds a price the way this instrument is normally quoted.
    ///
    /// **For showing to a human only.** Never round before comparing a price
    /// to a level.
    pub fn format_price(&self, price: Price) -> String {
        price.round_for_display(self.digits).to_string()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}
