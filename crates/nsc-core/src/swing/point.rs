//! One swing high or low.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::error::SwingError;
use super::kind::SwingKind;

/// One swing high or low.
///
/// **It carries two times on purpose**, and mixing them up is the single
/// easiest way to produce a backtest that looks wonderful and cannot be
/// traded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Swing {
    kind: SwingKind,

    /// The candle the swing sits on — where you would draw it.
    bar_time: DateTime<Utc>,

    /// **The first moment you could have known this was a swing.**
    ///
    /// Always later than `bar_time`, and how much later is not fixed: a swing
    /// is knowable at the candle where the pullback proved it, which is
    /// sometimes two candles and sometimes thirty.
    confirmed_at: DateTime<Utc>,

    /// The high, for a swing high. The low, for a swing low. **Wick
    /// included** — the swing sits at the extreme of the candle however long
    /// the wick is.
    price: Decimal,
}

impl Swing {
    /// **Refuses a swing that claims to be known before, or at, the candle it
    /// sits on.**
    ///
    /// That can never be true. You need candles *after* a peak to know it was
    /// a peak. If this check ever fires, whatever built the swing has a
    /// lookahead bug — and those do not announce themselves any other way.
    pub fn new(
        kind: SwingKind,
        bar_time: DateTime<Utc>,
        confirmed_at: DateTime<Utc>,
        price: Decimal,
    ) -> Result<Self, SwingError> {
        if confirmed_at <= bar_time {
            return Err(SwingError::KnownTooSoon {
                bar_time: bar_time.to_string(),
                confirmed_at: confirmed_at.to_string(),
            });
        }

        Ok(Swing {
            kind,
            bar_time,
            confirmed_at,
            price,
        })
    }

    pub fn kind(&self) -> SwingKind {
        self.kind
    }

    /// Where you would draw it.
    pub fn bar_time(&self) -> DateTime<Utc> {
        self.bar_time
    }

    /// The first moment it could have been known.
    ///
    /// **This is the one a backtest must use.** Reading a swing at its
    /// `bar_time` is using price the market had not printed yet.
    pub fn confirmed_at(&self) -> DateTime<Utc> {
        self.confirmed_at
    }

    pub fn price(&self) -> Decimal {
        self.price
    }

    /// Could this swing have been used at that moment?
    pub fn known_by(&self, now: DateTime<Utc>) -> bool {
        self.confirmed_at <= now
    }
}
