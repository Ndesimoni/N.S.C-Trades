//! The watcher that stands at one moment in the run.

use chrono::{DateTime, Utc};
use nsc_core::candle::Candle;
use nsc_core::level::Level;
use nsc_core::swing::Swing;
use nsc_core::timeframe::{DayBoundary, Timeframe};
use nsc_data::events::BarClosed;

use crate::error::BacktestError;

/// Stands at one moment and refuses anything the analysis could not have known
/// at it.
///
/// Make one for each bar, then pass everything the analysis reads through it.
///
/// ```ignore
/// let guard = Guard::at(&bar, boundary)?;
/// let swing = guard.swing(swing, Timeframe::H4)?;  // gives it back, or kills the run
/// ```
///
/// ## The moment is a real clock time, not a stamp
///
/// This is the part that is easy to get wrong, and it was wrong here first.
///
/// Everything in this project is stamped with its candle's **open** time. A
/// 4-hour candle that runs 13:00 to 17:00 is stamped 13:00, and so is a swing
/// confirmed by it.
///
/// But that swing was not knowable at 13:00. Nobody knew what that candle
/// would do until 17:00, when it finished.
///
/// So the guard holds the moment the bar **finished** — 17:00 — and works out
/// the same thing for whatever it is handed. Compare stamp against stamp and
/// the 4-hour looks like it arrived four hours early: at 17:00 it would throw
/// out a 15-minute swing from 16:45 that had genuinely happened, and let
/// through 4-hour readings that had not.
///
/// That is why the timeframe has to be passed in. A stamp on its own does not
/// say when it became true.
///
/// ## Why it hands the thing back instead of returning nothing
///
/// `guard.check(swing)?; use(swing);` is two lines, and the day someone
/// deletes the first one the run goes quiet rather than loud.
///
/// Handing the value back makes the guard the only way to get it. You cannot
/// forget a step you have to take to hold the thing at all.
///
/// ## What it cannot catch
///
/// Something the analysis never shows it. The guard is a gate, not a search —
/// it only sees what walks through. So the rule is that reads go through the
/// guard, and this type just makes that rule cheap to follow.
#[derive(Debug, Clone, Copy)]
pub struct Guard {
    now: DateTime<Utc>,
    boundary: DayBoundary,
}

impl Guard {
    /// Stand at the moment this bar finished.
    ///
    /// Not the moment it opened. A 4-hour bar stamped 13:00 tells you nothing
    /// until 17:00.
    pub fn at(bar: &BarClosed, boundary: DayBoundary) -> Result<Self, BacktestError> {
        let now = bar.timeframe().candle_end(bar.at(), &boundary)?;

        Ok(Self { now, boundary })
    }

    /// Stand at a clock time directly. For tests, and for the odd check with no
    /// bar in hand.
    pub fn standing_at(now: DateTime<Utc>, boundary: DayBoundary) -> Self {
        Self { now, boundary }
    }

    /// The clock time being stood at.
    pub fn now(self) -> DateTime<Utc> {
        self.now
    }

    /// Let a swing through, or kill the run.
    ///
    /// `timeframe` is the chart it was found on. A swing is knowable once the
    /// candle that confirmed it has closed — not when that candle opened, and
    /// certainly not at the peak itself, which nobody could have called at the
    /// time.
    pub fn swing(self, swing: Swing, timeframe: Timeframe) -> Result<Swing, BacktestError> {
        let knowable_at = timeframe.candle_end(swing.confirmed_at(), &self.boundary)?;

        if knowable_at <= self.now {
            return Ok(swing);
        }

        Err(self.caught(
            format!(
                "the {} swing {} at {}",
                timeframe,
                if swing.is_high() { "high" } else { "low" },
                swing.price()
            ),
            knowable_at,
        ))
    }

    /// Let a level through, or kill the run.
    ///
    /// No timeframe argument — a level already knows which chart it came from.
    pub fn level(self, level: Level) -> Result<Level, BacktestError> {
        let timeframe = level.timeframe();
        let knowable_at = timeframe.candle_end(level.confirmed_at(), &self.boundary)?;

        if knowable_at <= self.now {
            return Ok(level);
        }

        Err(self.caught(
            format!("the {timeframe} level at {}", level.band().centre()),
            knowable_at,
        ))
    }

    /// Let a candle through, or kill the run.
    ///
    /// Refuses two different mistakes with the same answer: a candle that had
    /// not closed by now, and a candle still marked as forming. Both are prices
    /// the market had not printed yet.
    pub fn candle(self, candle: &Candle, timeframe: Timeframe) -> Result<(), BacktestError> {
        if !candle.is_complete() {
            return Err(self.caught(
                format!(
                    "the unfinished {timeframe} candle opening {}",
                    candle.open_time()
                ),
                candle.open_time(),
            ));
        }

        let knowable_at = timeframe.candle_end(candle.open_time(), &self.boundary)?;

        if knowable_at > self.now {
            return Err(self.caught(
                format!("the {timeframe} candle opening {}", candle.open_time()),
                knowable_at,
            ));
        }

        Ok(())
    }

    /// Let a whole list of swings through. Stops at the first bad one — the run
    /// is over either way, and the first one is the one worth reading.
    pub fn swings(
        self,
        swings: &[Swing],
        timeframe: Timeframe,
    ) -> Result<Vec<Swing>, BacktestError> {
        swings.iter().map(|s| self.swing(*s, timeframe)).collect()
    }

    /// Let a whole list of levels through. They may be from mixed timeframes;
    /// each one is judged on its own.
    pub fn levels(self, levels: &[Level]) -> Result<Vec<Level>, BacktestError> {
        levels.iter().map(|l| self.level(*l)).collect()
    }

    fn caught(self, what: String, knowable_at: DateTime<Utc>) -> BacktestError {
        BacktestError::LookaheadDetected {
            what,
            now: self.now,
            knowable_at,
        }
    }
}
