//! Turning "EUR/USD" into something IBKR will accept.
//!
//! **The names do not match and never will.** His config says `EUR/USD`
//! because that is what a chart says. IBKR wants a symbol, a currency, a
//! security type and an exchange, and gets none of them from that string on
//! its own.
//!
//! One place does the translation, so a pair that will not resolve is one bug
//! in one file rather than a surprise at four call sites.

use ibapi::contracts::{Contract, Currency, Exchange, SecurityType, Symbol};

use super::error::IbkrError;

/// Where spot forex is traded at IBKR.
const FOREX_AT: &str = "IDEALPRO";

/// Where everything else is routed. SMART lets IBKR pick the venue.
const ELSEWHERE: &str = "SMART";

/// The contract for one of his pairs.
///
/// **Gold is not a forex pair, and that catches people out.** `XAU/USD` looks
/// exactly like `EUR/USD` and is a completely different kind of instrument to
/// IBKR — a commodity, routed through SMART, needing a metals market data
/// subscription that spot forex does not give you.
pub fn for_symbol(symbol: &str) -> Result<Contract, IbkrError> {
    let (base, quote) = split(symbol)?;

    // Metals are commodities here, whatever the slash makes them look like.
    if matches!(base, "XAU" | "XAG" | "XPT" | "XPD") {
        return Ok(Contract {
            symbol: Symbol::from(format!("{base}{quote}")),
            security_type: SecurityType::Commodity,
            exchange: Exchange::from(ELSEWHERE),
            currency: Currency::from(quote),
            ..Contract::default()
        });
    }

    Ok(Contract {
        symbol: Symbol::from(base),
        security_type: SecurityType::ForexPair,
        exchange: Exchange::from(FOREX_AT),
        currency: Currency::from(quote),
        ..Contract::default()
    })
}

/// `"EUR/USD"` into `("EUR", "USD")`.
///
/// Refused rather than guessed at. A symbol that does not split is a typo in
/// a config file, and quietly asking IBKR for something else would report
/// prices for an instrument he never asked about.
fn split(symbol: &str) -> Result<(&str, &str), IbkrError> {
    let Some((base, quote)) = symbol.split_once('/') else {
        return Err(IbkrError::Refused {
            symbol: symbol.to_string(),
            why: "not written as BASE/QUOTE, so there is no way to know what it means".into(),
        });
    };

    if base.is_empty() || quote.is_empty() {
        return Err(IbkrError::Refused {
            symbol: symbol.to_string(),
            why: "one side of the slash is empty".into(),
        });
    }

    Ok((base, quote))
}
