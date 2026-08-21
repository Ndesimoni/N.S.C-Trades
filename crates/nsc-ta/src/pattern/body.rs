//! A candle's body, as prices rather than as a share.
//!
//! **Patterns compare bodies in PRICE, shapes compare them as ratios.** An
//! engulfing is one body covering another on the chart — that question cannot
//! be answered from two ratios, because two candles of different heights can
//! have the same body share and not overlap at all.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

/// Where a candle's body sits, and which way it went.
#[derive(Debug, Clone, Copy)]
pub(super) struct Body {
    pub(super) top: Decimal,
    pub(super) bottom: Decimal,
    pub(super) up: bool,
}

impl Body {
    pub(super) fn of(bar: &Bar) -> Self {
        Body {
            top: bar.open.max(bar.close),
            bottom: bar.open.min(bar.close),
            up: bar.close >= bar.open,
        }
    }

    /// How tall it is, in price.
    pub(super) fn size(self) -> Decimal {
        self.top - self.bottom
    }

    /// How much of its own candle it takes.
    ///
    /// **Nought for a flat candle, not a division by zero.** The feed sends
    /// weekend and holiday candles and on gold they are flat — seven of 4,165
    /// have the high, low, open and close all the same number.
    pub(super) fn share(self, bar: &Bar) -> Decimal {
        let range = bar.high - bar.low;

        if range <= Decimal::ZERO {
            return Decimal::ZERO;
        }

        self.size() / range
    }

    /// Is this body completely inside that one?
    pub(super) fn inside(self, other: Body) -> bool {
        self.top <= other.top && self.bottom >= other.bottom
    }

    /// Does this body completely cover that one?
    pub(super) fn covers(self, other: Body) -> bool {
        other.inside(self)
    }
}
