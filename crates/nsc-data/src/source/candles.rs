//! The one thing every feed must be able to do.

use nsc_core::candle::Bar;

use super::Interval;

/// Somewhere candles come from.
///
/// **Four promises, and they are the whole reason this trait exists:**
///
/// ```text
///     stamps are UTC              and mark when the candle STARTED
///     the newest may be unfinished and the caller is told to ask the clock
///     prices are mid              never one side of the spread
///     boundaries are the feed's   never worked out from the interval
/// ```
///
/// The last two are where feeds actually differ, and neither errors when it
/// is wrong. A daily boundary four hours out still hands back a perfectly
/// good candle. It is just not the candle on his chart, and the first thing
/// he sees is a level in the wrong place.
pub trait MarketDataSource {
    /// What went wrong, and whether asking again would help.
    type Trouble: std::error::Error + Send + Sync + 'static;

    /// The most recent candles for a pair, **newest first**.
    ///
    /// The newest is usually the one still forming. Which have finished is
    /// asked of the clock by `Bar::finished_by` — never worked out from
    /// position in the list. Ask at 16:00:02 and you get either the 16:00
    /// candle already open, if a price has landed, or the 15:00 one now
    /// finished, if none has. Position is right most of the time and wrong the
    /// rest, which is worse than wrong always, because you stop checking.
    fn candles(
        &self,
        symbol: &str,
        interval: Interval,
        count: usize,
    ) -> impl Future<Output = Result<Vec<Bar>, Self::Trouble>> + Send;
}
