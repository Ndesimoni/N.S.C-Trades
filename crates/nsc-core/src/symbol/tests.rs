use rust_decimal::Decimal;

use super::*;
use crate::error::CoreError;
use crate::price::{Pips, Price, PriceDistance};

fn currency(code: &str) -> Currency {
    Currency::new(code).expect("valid code")
}

fn eurusd() -> Symbol {
    Symbol::new(
        "EURUSD",
        AssetClass::Forex,
        Decimal::new(1, 4), // 0.0001
        5,
        Pips::new(Decimal::new(20, 1)), // 2.0
        Some(currency("EUR")),
        Some(currency("USD")),
    )
    .expect("valid symbol")
}

fn us30() -> Symbol {
    Symbol::new(
        "US30",
        AssetClass::Index,
        Decimal::ONE, // one point
        1,
        Pips::new(Decimal::new(40, 1)), // 4.0
        None,
        Some(currency("USD")),
    )
    .expect("valid symbol")
}

#[test]
fn currency_codes_are_checked() {
    assert_eq!(currency("usd").code(), "USD");
    assert!(Currency::new("USDD").is_err());
    assert!(Currency::new("U5D").is_err());
}

#[test]
fn asset_classes_parse_from_config_strings() {
    assert_eq!("metal".parse::<AssetClass>(), Ok(AssetClass::Metal));
    assert_eq!("Index".parse::<AssetClass>(), Ok(AssetClass::Index));
    assert!("crypto".parse::<AssetClass>().is_err());
}

#[test]
fn a_zero_pip_size_is_refused_at_startup() {
    let result = Symbol::new(
        "EURUSD",
        AssetClass::Forex,
        Decimal::ZERO,
        5,
        Pips::new(Decimal::TWO),
        None,
        None,
    );

    assert!(matches!(result, Err(CoreError::InvalidPipSize { .. })));
}

#[test]
fn the_news_filter_can_ask_about_a_currency() {
    let pair = eurusd();

    assert!(pair.involves(&currency("USD")));
    assert!(pair.involves(&currency("EUR")));
    assert!(!pair.involves(&currency("GBP")));
}

#[test]
fn an_index_has_no_base_currency() {
    let index = us30();

    assert_eq!(index.base(), None);
    assert!(index.involves(&currency("USD")));
}

#[test]
fn a_distance_becomes_pips_for_this_instrument() {
    let pair = eurusd();
    let gap = PriceDistance::new(Decimal::new(50, 4)); // 0.0050

    let pips = pair.to_pips(gap).expect("pip size is valid");
    assert_eq!(pips.value(), Decimal::new(50, 0)); // 50 pips
}

#[test]
fn a_wide_spread_kills_the_setup() {
    let pair = eurusd(); // limit is 2.0

    assert!(pair.spread_is_acceptable(Pips::new(Decimal::new(15, 1)))); // 1.5
    assert!(!pair.spread_is_acceptable(Pips::new(Decimal::new(35, 1)))); // 3.5
}

#[test]
fn prices_are_shown_the_way_the_instrument_is_quoted() {
    let pair = eurusd();
    let price = Price::new(Decimal::new(108505, 5)); // 1.08505

    assert_eq!(pair.format_price(price), "1.08505");
}
