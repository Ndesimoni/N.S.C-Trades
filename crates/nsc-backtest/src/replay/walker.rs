//! Walking a history one candle at a time.

use std::sync::Arc;

use nsc_core::candle::Candle;
use nsc_core::symbol::Symbol;
use nsc_core::timeframe::{DayBoundary, Timeframe};
use nsc_data::events::BarClosed;
use nsc_ta::aggregate::Aggregator;

use crate::error::BacktestError;

/// Feeds a history through, one candle at a time, the way the live bot gets
/// them.
///
/// Give it the file as it arrived — 15-minute candles, say — and the bigger
/// timeframes to build. Each base candle goes in, and out come the bars that
/// finished because of it: the base one always, plus any bigger candle it
/// completed.
///
/// ## Why this is not just a loop over the file
///
/// A 15-minute candle at 16:45 also finishes the 4-hour candle that started at
/// 13:00. The live bot learns both at that moment, so a replay has to as well
/// — otherwise the backtester sees a different sequence from the bot and the
/// results stop describing it.
///
/// ## The order matters
///
/// Bars come out **biggest timeframe first**. At 16:00 the 30-minute, 1-hour
/// and 4-hour can all finish together, and the smaller ones read the bigger
/// ones for context. Run them the other way round and the 30-minute reads a
/// 4-hour that has not been updated yet.
///
/// That is `evaluate_largest_first` in `config/app.toml`, and it is not
/// optional — the wrong order gives different answers on the same candles,
/// and it would differ between the backtester and the bot.
pub struct Replay {
    symbol: Arc<Symbol>,
    base: Timeframe,
    builders: Vec<(Timeframe, Aggregator)>,
}

impl Replay {
    /// `derived` are the bigger timeframes to build. Any that is not bigger
    /// than the base is refused rather than skipped — asking for 4-hour
    /// candles out of a daily file is a mistake, not a no-op, and quietly
    /// ignoring it would leave you wondering why no 4-hour bars ever arrived.
    pub fn new(
        symbol: Arc<Symbol>,
        base: Timeframe,
        derived: &[Timeframe],
        boundary: DayBoundary,
    ) -> Result<Self, BacktestError> {
        let mut wanted: Vec<Timeframe> = derived.iter().copied().filter(|tf| *tf != base).collect();

        wanted.sort_unstable();
        wanted.dedup();

        let mut builders = Vec::with_capacity(wanted.len());

        for timeframe in wanted {
            builders.push((timeframe, Aggregator::new(base, timeframe, boundary)?));
        }

        Ok(Self {
            symbol,
            base,
            builders,
        })
    }

    /// Takes the next candle from the file and gives back every bar that
    /// finished because of it.
    ///
    /// Always at least one — the base candle itself. Sometimes more, when it
    /// also completed an hour, a day or a week.
    ///
    /// Biggest timeframe first, so context is fresh before anything reads it.
    pub fn feed(&mut self, candle: &Candle) -> Result<Vec<BarClosed>, BacktestError> {
        let mut finished = Vec::new();

        // Build the bigger ones first so they can be emitted before the base.
        for (timeframe, builder) in self.builders.iter_mut() {
            if let Some(bigger) = builder.update(candle)? {
                finished.push(BarClosed::new(
                    Arc::clone(&self.symbol),
                    *timeframe,
                    bigger,
                )?);
            }
        }

        // Biggest first. The smaller timeframes read the bigger ones for
        // context, so the bigger ones have to have moved already.
        finished.sort_by_key(|bar| std::cmp::Reverse(bar.timeframe()));

        finished.push(BarClosed::new(
            Arc::clone(&self.symbol),
            self.base,
            *candle,
        )?);

        Ok(finished)
    }
}
