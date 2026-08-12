//! Things that can go wrong, as types.
//!
//! The clean crates return errors instead of crashing. The backtester runs
//! this code across years of candles, so a crash on one bad candle would kill
//! an entire test run. Bad input should be reported and skipped, not fatal.

use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CoreError {
    /// A settings mistake. Stop rather than skip.
    #[error("pip size is zero, so a distance cannot be converted to pips")]
    ZeroPipSize,

    /// A flat or malformed stretch of candles. Skip the setup, keep running.
    #[error("ATR is zero or negative, so a distance cannot be measured in normal candles")]
    ZeroAtr,

    /// The gap between round numbers was set to zero or less.
    ///
    /// A settings mistake. With a step of zero every price is round, so "price
    /// is at a round number" is always true — a check that always says yes is
    /// the same as no check at all. Stop rather than skip.
    #[error("a round-number step of {step} cannot be used")]
    InvalidRoundStep { step: String },

    /// The ladder of round-number steps for an instrument is unusable.
    ///
    /// A settings mistake. Stop rather than skip — out of order, how round a
    /// price counts as would depend on the order someone typed the settings.
    #[error("the round-number steps cannot be used: {detail}")]
    InvalidRoundLadder { detail: String },

    /// A float that was infinity or NaN.
    #[error("the value {value} cannot be represented as a decimal")]
    NotRepresentable { value: f64 },

    /// A ratio could not be turned into a floating point number. Means the
    /// ATR was absurdly small — the data is broken, not the maths.
    #[error("a ratio could not be represented as a floating point number")]
    RatioNotRepresentable,

    /// The daily close time in `config/app.toml` is not a real time of day.
    /// A settings mistake — stop rather than skip.
    #[error("{hour:02}:{minute:02} is not a real time of day")]
    InvalidTimeOfDay { hour: u32, minute: u32 },

    /// A timeframe name in the config is not one we know.
    #[error("'{text}' is not a timeframe this system knows")]
    UnknownTimeframe { text: String },

    /// The local time asked for does not exist, because the clocks jumped
    /// forward over it. Only happens if the daily close is set to something
    /// like 02:30, which is inside the gap in many timezones.
    #[error("{local} does not exist in {tz} — the clocks skip over it")]
    LocalTimeDoesNotExist { local: String, tz: String },

    /// Ran off the end of the calendar. Realistically impossible; here so
    /// that date maths never has to panic.
    #[error("the date is outside the range this program can handle")]
    DateOutOfRange,

    /// An instrument in `config/symbols.toml` has no name.
    #[error("an instrument was configured with an empty name")]
    EmptySymbolName,

    /// Pip size is missing or negative. Without it, no stop distance means
    /// anything. A settings mistake — stop rather than skip.
    #[error("{symbol} has a pip size of {pip_size}, which cannot be used")]
    InvalidPipSize { symbol: String, pip_size: String },

    /// Currency codes are three letters, like USD or EUR.
    #[error("'{text}' is not a three-letter currency code")]
    InvalidCurrencyCode { text: String },

    /// A class in `config/symbols.toml` is not one we know.
    #[error("'{text}' is not an asset class this system knows")]
    UnknownAssetClass { text: String },

    /// The feed sent something that cannot be a real candle — a high below
    /// the low, or an open outside the two.
    ///
    /// Skip the candle and carry on. One bad row must not kill a backfill
    /// that has been running for an hour.
    #[error("candle at {open_time} cannot be real: {detail}")]
    ImpossibleCandle {
        open_time: DateTime<Utc>,
        detail: String,
    },

    /// Candles arrived out of order, or the same one arrived twice in stored
    /// history.
    ///
    /// A series that is not in time order silently breaks every swing, level
    /// and trendline built from it, and nothing else would ever report it.
    #[error("candle at {arriving} does not come after the last one at {last}")]
    CandlesOutOfOrder {
        arriving: DateTime<Utc>,
        last: DateTime<Utc>,
    },

    /// A swing was marked as known before the candle it sits on.
    ///
    /// A swing high at candle 100 cannot be known until a few candles later —
    /// you have to see price come back down first. Confirming it any earlier
    /// means using knowledge you did not have, and every backtest built on
    /// that is fiction.
    #[error("a swing at {bar_time} cannot be known at {confirmed_at}, which is not later")]
    SwingKnownTooEarly {
        bar_time: DateTime<Utc>,
        confirmed_at: DateTime<Utc>,
    },

    /// A level was described in a way that cannot be real — a band whose top
    /// is below its bottom, no touches at all, or a last touch that comes
    /// before the first one.
    ///
    /// Whatever built the level has a bug. Stop rather than skip.
    #[error("this level cannot be real: {detail}")]
    ImpossibleLevel { detail: String },

    /// A level was marked as known before the swing that finished it.
    ///
    /// A level with three touches is not knowable until the third touch has
    /// confirmed as a swing — which is always a few candles after the candle
    /// it sits on. Drawing it any earlier is drawing a level out of prices
    /// the market had not printed yet.
    #[error(
        "a level last touched at {last_touch} cannot be known at {confirmed_at}, which is not later"
    )]
    LevelKnownTooEarly {
        last_touch: DateTime<Utc>,
        confirmed_at: DateTime<Utc>,
    },

    /// A break of structure was described in a way that cannot be real — one
    /// that happens before the extreme it breaks, or one where price never
    /// actually carried past.
    ///
    /// Whatever built it read the chart backwards. Stop rather than skip.
    #[error("this break of structure cannot be real: {detail}")]
    ImpossibleStructureBreak { detail: String },

    /// A Fibonacci move was described in a way that cannot be real — one that
    /// ends before it starts, or one that went nowhere.
    ///
    /// Whatever built it has a bug. Stop rather than skip.
    #[error("this move cannot be measured: {detail}")]
    ImpossibleRetracement { detail: String },

    /// A candlestick pattern was described in a way that cannot be real.
    ///
    /// Whatever built it has a bug. Stop rather than skip.
    #[error("this candlestick pattern cannot be real: {detail}")]
    ImpossiblePattern { detail: String },

    /// Something tried to change a candle that had already closed.
    ///
    /// Once a candle is complete it is history. If history can be rewritten,
    /// you can run the same backtest twice and get two different answers,
    /// with no way to tell which one was right.
    #[error("the candle at {open_time} has already closed and cannot be changed")]
    HistoryRewrite { open_time: DateTime<Utc> },
}
