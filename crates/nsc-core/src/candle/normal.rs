//! How big a normal candle is.
//!
//! **The number every distance in this project is measured against.** How
//! thick a band is, how close counts as arriving, whether a candle is big
//! enough to mean anything — all of it is a multiple of this.
//!
//! It is here rather than on `Bar` because it is a fact about a RUN of
//! candles, not about one. A single candle cannot answer it.

use super::Bar;
use rust_decimal::Decimal;

/// How big a normal candle is, over the last `count` of them.
///
/// **True range, not high minus low.** A candle that gapped away from the one
/// before it moved further than its own body shows, and ignoring that makes
/// every band too thin on exactly the days price is moving most.
///
/// `bars` are oldest first.
pub fn normal_candle(bars: &[&Bar], count: usize) -> Option<Decimal> {
    if bars.len() < 2 {
        return None;
    }

    let ranges: Vec<Decimal> = bars
        .windows(2)
        .map(|pair| {
            let (before, now) = (pair[0], pair[1]);

            let high_low = now.high - now.low;
            let gap_up = (now.high - before.close).abs();
            let gap_down = (now.low - before.close).abs();

            high_low.max(gap_up).max(gap_down)
        })
        .collect();

    let recent = &ranges[ranges.len().saturating_sub(count)..];
    if recent.is_empty() {
        return None;
    }

    Some(recent.iter().sum::<Decimal>() / Decimal::from(recent.len()))
}
