//! Candles that are there but never moved.

use chrono::{DateTime, Utc};
use nsc_core::candle::Candle;
use nsc_core::price::Price;

/// A run of candles in a row that all had no range at all.
///
/// High, low, open and close at one price. Nothing traded away from it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlatRun {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    candles: usize,
    price: Price,
}

impl FlatRun {
    /// Open time of the first flat candle.
    pub fn from(self) -> DateTime<Utc> {
        self.from
    }

    /// Open time of the last flat candle.
    pub fn to(self) -> DateTime<Utc> {
        self.to
    }

    /// How many candles in a row.
    pub fn candles(self) -> usize {
        self.candles
    }

    /// The one price they all sat at.
    ///
    /// A run only exists while the price stays put, so there is exactly one.
    pub fn price(self) -> Price {
        self.price
    }
}

/// Finds every stretch of `min_run` or more candles in a row with no range.
///
/// ## Why runs, and not single candles
///
/// One flat 15-minute candle is nothing. It happens on a quiet Sunday evening
/// and means only that nothing traded for a quarter of an hour.
///
/// Twenty in a row is a different animal. Either the market was shut and the
/// broker filled the hours with the last price, or the feed froze. Either way
/// the analysis reads that shelf as a real price level being defended, and it
/// was never defended by anybody.
///
/// This project has already paid for that once: two flat candles at the left
/// edge of a history invented a swing on every instrument, and every test was
/// green.
///
/// ## Where `min_run` comes from
///
/// The caller, out of `config/`. What counts as too long is something a trader
/// tunes per instrument — gold overnight is not USDCAD overnight — so it does
/// not belong hardcoded here.
///
/// A `min_run` of 0 or 1 reports every single flat candle, which on a
/// 15-minute history is thousands of rows of noise.
pub fn find_flat_runs(candles: &[Candle], min_run: usize) -> Vec<FlatRun> {
    let mut runs = Vec::new();
    let mut start: Option<usize> = None;

    for (i, candle) in candles.iter().enumerate() {
        let flat = candle.high() == candle.low();

        match (flat, start) {
            (true, None) => start = Some(i),

            // A flat candle at a *different* price ends the shelf and starts a
            // new one. Both candles have no range of their own, but price
            // moved between them — so calling it one run at one price would be
            // a made-up number in the report.
            (true, Some(first)) if candle.close() != candles[first].close() => {
                push_run(&mut runs, candles, first, i - 1, min_run);
                start = Some(i);
            }

            (false, Some(first)) => {
                push_run(&mut runs, candles, first, i - 1, min_run);
                start = None;
            }

            _ => {}
        }
    }

    // A run that is still open when the candles end is still a run. Forgetting
    // this loses the worst case — a feed that froze and never came back.
    if let Some(first) = start {
        push_run(&mut runs, candles, first, candles.len() - 1, min_run);
    }

    runs
}

fn push_run(runs: &mut Vec<FlatRun>, candles: &[Candle], first: usize, last: usize, min: usize) {
    let length = last - first + 1;

    if length < min.max(1) {
        return;
    }

    runs.push(FlatRun {
        from: candles[first].open_time(),
        to: candles[last].open_time(),
        candles: length,
        price: candles[first].close(),
    });
}
