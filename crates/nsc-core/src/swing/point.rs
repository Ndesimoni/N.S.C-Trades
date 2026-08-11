//! One swing high or low.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::kind::SwingKind;
use crate::error::CoreError;
use crate::price::Price;

/// One swing high or low.
///
/// Carries two times on purpose. Mixing them up is the single easiest way to
/// produce a backtest that looks wonderful and cannot be traded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Swing {
    kind: SwingKind,

    /// The candle the swing sits on — where you would draw it.
    bar_time: DateTime<Utc>,

    /// The first moment you could have known this was a swing.
    ///
    /// Always later than `bar_time`. How much later depends on the lookback
    /// setting in `config/ta.toml`.
    confirmed_at: DateTime<Utc>,

    /// The high, for a swing high. The low, for a swing low.
    price: Price,
}

impl Swing {
    /// Refuses a swing that claims to be known before, or at, the candle it
    /// sits on.
    ///
    /// That can never be true. You need candles *after* a peak to know it was
    /// a peak. If this check ever fires, whatever built the swing has a
    /// lookahead bug — and lookahead bugs do not announce themselves any
    /// other way.
    pub fn new(
        kind: SwingKind,
        bar_time: DateTime<Utc>,
        confirmed_at: DateTime<Utc>,
        price: Price,
    ) -> Result<Self, CoreError> {
        if confirmed_at <= bar_time {
            return Err(CoreError::SwingKnownTooEarly {
                bar_time,
                confirmed_at,
            });
        }

        Ok(Self {
            kind,
            bar_time,
            confirmed_at,
            price,
        })
    }

    pub fn kind(self) -> SwingKind {
        self.kind
    }

    /// Where the swing is on the chart.
    pub fn bar_time(self) -> DateTime<Utc> {
        self.bar_time
    }

    /// The first moment you could have known about it.
    pub fn confirmed_at(self) -> DateTime<Utc> {
        self.confirmed_at
    }

    pub fn price(self) -> Price {
        self.price
    }

    pub fn is_high(self) -> bool {
        self.kind.is_high()
    }

    pub fn is_low(self) -> bool {
        self.kind.is_low()
    }

    /// Could you have known about this swing at `now`?
    ///
    /// **Call this before using a swing for anything.** Drawing a level from
    /// a swing you had not seen yet is the quiet mistake that makes a
    /// backtest look far better than the live bot ever will.
    ///
    /// `nsc-backtest::guards` exists to catch anyone who forgets — but it is
    /// much cheaper to ask here.
    pub fn is_known_at(self, now: DateTime<Utc>) -> bool {
        now >= self.confirmed_at
    }
}
