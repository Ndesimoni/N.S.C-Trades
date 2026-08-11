use rust_decimal::Decimal;

use super::*;
use crate::error::CoreError;

fn price(mantissa: i64, scale: u32) -> Price {
    Price::new(Decimal::new(mantissa, scale))
}

#[test]
fn subtracting_two_prices_gives_a_distance() {
    let gap = price(10850, 4) - price(10800, 4);
    assert_eq!(gap.value(), Decimal::new(50, 4));
}

#[test]
fn pips_round_trip() {
    let pip_size = Decimal::new(1, 4); // 0.0001, EURUSD
    let gap = price(10850, 4) - price(10800, 4);

    let pips = gap.to_pips(pip_size).expect("pip size is not zero");
    assert_eq!(pips.value(), Decimal::new(50, 0));

    assert_eq!(pips.to_distance(pip_size), gap);
}

#[test]
fn a_distance_in_normal_candles() {
    let gap = PriceDistance::new(Decimal::new(30, 4)); // 0.0030
    let atr = PriceDistance::new(Decimal::new(10, 4)); // 0.0010

    assert_eq!(
        gap.to_atr_multiple(atr).expect("atr above zero").value(),
        3.0
    );
}

#[test]
fn flat_market_does_not_panic() {
    let gap = PriceDistance::new(Decimal::new(30, 4));
    let flat = PriceDistance::new(Decimal::ZERO);

    assert_eq!(gap.to_atr_multiple(flat), Err(CoreError::ZeroAtr));
}

#[test]
fn a_stop_buffer_becomes_a_real_distance() {
    let atr = PriceDistance::new(Decimal::new(10, 4)); // 0.0010
    let buffer = AtrMultiple::new(0.3);

    let distance = buffer.to_distance(atr).expect("0.3 is representable");
    assert_eq!(distance.value(), Decimal::new(3, 4)); // 0.3 x 0.0010 = 0.0003

    let stop = price(10800, 4) - distance;
    assert_eq!(stop.value(), Decimal::new(107970, 5)); // 1.0800 - 0.0003
}
