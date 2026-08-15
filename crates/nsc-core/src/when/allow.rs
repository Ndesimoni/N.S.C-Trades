//! What the bot may say at this moment.

use chrono::{DateTime, Duration, Utc};

use super::{Rules, into_day, trading_day};

/// Three states, because "do not trade" and "do not speak" are different.
///
/// Collapsing them would either silence a day he wants to watch, or suggest
/// trades in the hours he never takes them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Allowed {
    /// Everything, including a trade.
    Anything,

    /// What is happening at his zones, but no trade suggested.
    ///
    /// The first hours of a new day, and Friday. Price reaching a zone and a
    /// candle closing in one are **information**; a signal is an instruction,
    /// and only the instruction is held back.
    WatchOnly,

    /// Nothing but the heartbeat.
    ///
    /// **And it means nothing** — no prices checked, no candles fetched, no
    /// queue building up to be dumped on him the next morning. Monday.
    Silence,
}

impl Allowed {
    /// May anything at all be sent, or watched for?
    pub fn says_anything(self) -> bool {
        self != Allowed::Silence
    }

    /// May a trade be suggested?
    pub fn may_trade(self) -> bool {
        self == Allowed::Anything
    }
}

/// What the bot may say at `now`.
///
/// **`now` is handed in, never read.** That is what lets the backtester run
/// these exact rules over 2019 by passing 2019 in.
pub fn allowed(now: DateTime<Utc>, rules: &Rules) -> Allowed {
    let day = trading_day(now, rules);

    if rules.silent_days.contains(&day) {
        return Allowed::Silence;
    }

    // The settle window is measured from the session's own open, so it is four
    // hours of the market being awake rather than four hours on a wall clock.
    if into_day(now, rules) < Duration::hours(rules.settle_hours) {
        return Allowed::WatchOnly;
    }

    if rules.no_new_trades.contains(&day) {
        return Allowed::WatchOnly;
    }

    Allowed::Anything
}
