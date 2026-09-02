//! **Why a candle did not become a signal.**
//!
//! `look` used to answer `Option<Signal>` — a signal, or nothing. Nothing is
//! the honest answer for the market and a useless one for the record.
//!
//! `CLAUDE.md`: *"Rejected setups get saved, not thrown away. Save which layer
//! rejected them. Those rows answer 'why did nothing fire this week?'"*
//!
//! **"Nothing fired this week" and "forty shapes printed and every one was
//! thrown out at the place test" are completely different problems**, and
//! today they are the same silence.
//!
//! Nothing here reaches anything. It is a word for a reason, and the driver
//! decides whether to write it down.

use nsc_ta::pattern::Pattern;
use rust_decimal::Decimal;

use super::shape::Traded;

/// Why this candle is not a signal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// **No shape at all**, which is nearly every candle.
    ///
    /// Carried as a variant so the caller can tell it apart, and deliberately
    /// NOT worth writing down — a row per quiet candle is the candle table
    /// again, in a table that is meant to be read. See [`Refused::worth_keeping`].
    NoShape,

    /// A shape printed, and it is **not one he trades**.
    ///
    /// `pattern/` names eight and he trades four. Tweezers, piercing, dark
    /// cloud and the star are on every candlestick page and not on his chart.
    NotHis { pattern: Pattern },

    /// **A shape he trades, with no level under it.** The one that matters.
    ///
    /// His own `nsc-bull` and `nsc-bear` measured with no level came back at
    /// 38% over 75 tries, where a coin flip is 50%. The level is what makes a
    /// shape worth anything, so this is a refusal and not a near miss — but it
    /// is also the single most interesting row in the table.
    NoLevel { shape: Traded, touching: Decimal },

    /// The shape could not be measured — fewer candles than it needs, or a
    /// normal candle of zero. **Not a near miss, nothing at all.**
    Unmeasurable { shape: Traded },
}

impl Refused {
    /// Which layer said no. The word the record stores.
    ///
    /// **The layer is the whole point.** It is what separates "the market was
    /// quiet" from "the shapes were there and the levels were not".
    pub fn layer(&self) -> &'static str {
        match self {
            Refused::NoShape | Refused::NotHis { .. } => "shape",
            Refused::NoLevel { .. } => "place",
            Refused::Unmeasurable { .. } => "measure",
        }
    }

    /// The specific test that failed, in one line he could read.
    pub fn why(&self) -> String {
        match self {
            Refused::NoShape => "no shape on this candle".into(),

            Refused::NotHis { pattern } => {
                // **`spoken()` and nothing of our own.** A second list of names
                // for the same eight shapes is a second thing to keep in step.
                format!("{} is not one of the four he trades", pattern.spoken())
            }

            Refused::NoLevel { shape, touching } => {
                format!(
                    "{} printed at {touching}, with no level near it",
                    shape.name()
                )
            }

            Refused::Unmeasurable { shape } => {
                format!("not enough candles to measure a {}", shape.name())
            }
        }
    }

    /// **Is this worth a row?**
    ///
    /// Everything except a quiet candle. Most candles have no shape at all,
    /// and writing one down would make a table far larger than the candles it
    /// describes while saying less — the candle is already stored, and "there
    /// was no shape on it" can be worked out from the candle any time.
    ///
    /// What cannot be worked out afterwards is the rest: a shape the rules
    /// refused, or one that printed nowhere near a level. Those depend on the
    /// settings that were live at that moment, and those change.
    pub fn worth_keeping(&self) -> bool {
        !matches!(self, Refused::NoShape)
    }
}
