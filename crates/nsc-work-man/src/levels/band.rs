//! Turning a line into a band.

use rust_decimal::Decimal;
use serde::Deserialize;

/// Which chart he drew it on. The colour follows from this, and so does the
/// thickness — they are his specification, not a design choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Timeframe {
    Weekly,
    Daily,
    #[serde(alias = "4-hour", alias = "4h")]
    H4,
}

impl Timeframe {
    /// The colour he draws it in. **This is a specification.**
    ///
    /// Drawing every level in one colour was done once already and the chart
    /// looked nothing like his.
    ///
    /// The card turns `black` into white, because the card is dark and his
    /// chart is white. **Same role, other background** — the weekly is meant to
    /// be the heaviest mark on the chart, and on a dark ground that is white.
    pub fn colour(self) -> &'static str {
        match self {
            Timeframe::Weekly => "black",
            Timeframe::Daily => "blue",
            Timeframe::H4 => "yellow",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Timeframe::Weekly => "weekly",
            Timeframe::Daily => "daily",
            Timeframe::H4 => "4-hour",
        }
    }
}

/// One level, as a band.
#[derive(Debug, Clone, Copy)]
pub struct Band {
    pub timeframe: Timeframe,
    pub price: Decimal,
    pub top: Decimal,
    pub bottom: Decimal,
}

impl Band {
    /// Builds the band around the line he drew.
    ///
    /// `candle` is how big a normal candle is on that timeframe — its ATR.
    /// `share` is how much of one the band is worth, from
    /// `config/levels.toml`.
    ///
    /// The line is the **middle**. Half the thickness above, half below.
    pub fn around(timeframe: Timeframe, price: Decimal, candle: Decimal, share: Decimal) -> Self {
        let half = candle * share / Decimal::TWO;

        Band {
            timeframe,
            price,
            top: price + half,
            bottom: price - half,
        }
    }

    /// Is this price inside the band?
    ///
    /// The whole question a level exists to answer.
    pub fn holds(&self, price: Decimal) -> bool {
        price >= self.bottom && price <= self.top
    }

    /// How thick it came out.
    pub fn thickness(&self) -> Decimal {
        self.top - self.bottom
    }
}
