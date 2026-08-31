//! Shared by the level tests.

use rust_decimal::Decimal;

use super::super::{Band, Side, Timeframe};
use crate::candle::Bar;

pub(super) fn d(text: &str) -> Decimal {
    text.parse().expect("a number")
}

/// A scratch folder of its own, so tests cannot tread on each other.
pub(super) fn scratch(name: &str) -> std::path::PathBuf {
    let folder = std::env::temp_dir().join(format!("nsc-levels-{name}"));
    let _ = std::fs::remove_dir_all(&folder);
    folder
}

/// A twentieth of whatever band it is asked about.
///
/// **A share, not a price, since 31 August 2026.** It was ten cents once — one
/// gold pip — which is 0.13% of the gold weekly band and 22% of an AUD/USD
/// daily one. The same setting meaning two different things is what broke it.
pub(super) fn share() -> Decimal {
    d("0.05")
}

/// The gold band he drew at 4094 — 4055.43 to 4132.57, about 77.15 thick.
///
/// A touch reaches **4136.43**, a twentieth of the band past the edge, and the
/// band goes quiet again only past **4144.15** — a tenth of the band clear of
/// where approaching ends.
pub(super) fn gold() -> Band {
    Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"))
}

/// His AUD/USD daily level at 0.71500, on a 49.3-pip daily candle.
///
/// The band comes out **22.7 pips** thick — 0.713865 to 0.716135.
pub(super) fn aussie() -> Band {
    Band::around(Timeframe::Daily, d("0.71500"), d("0.004935"), d("0.46"))
}

/// A candle that only has to say where it opened and where it closed. The high
/// and low are stretched to cover both, so it always touches what it crossed.
pub(super) fn bar(open: Decimal, close: Decimal) -> Bar {
    Bar {
        datetime: "2026-08-31 12:00:00".into(),
        open,
        high: open.max(close),
        low: open.min(close),
        close,
    }
}

/// The candle opened below the band.
pub(super) fn from_below() -> Option<Side> {
    Some(Side::Below)
}

/// The candle opened above it.
pub(super) fn from_above() -> Option<Side> {
    Some(Side::Above)
}
