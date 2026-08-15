//! The candle itself.

use super::CandleError;
use chrono::{DateTime, NaiveDateTime, TimeDelta, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::settings::INTERVAL_MINUTES;

/// What Twelve Data sends back for a time series request.
///
/// Only the part we need. Serde skips the `meta` block and anything else they
/// add later, so a new field on their side cannot break this.
#[derive(Debug, Deserialize)]
pub struct Series {
    pub values: Vec<Bar>,
}

/// One candle, in the feed's own words.
///
/// No volume field, because spot forex and gold have none — there is no
/// central exchange to count it. No rule in this project may ever use volume.
#[derive(Debug, Clone, Deserialize)]
pub struct Bar {
    pub datetime: String,

    // Prices arrive as text — "4394.68931" — and go straight into Decimal.
    // They never touch a float, not even for a moment.
    #[serde(with = "rust_decimal::serde::str")]
    pub open: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub high: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub low: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub close: Decimal,
}

impl Bar {
    /// When this candle opened.
    ///
    /// True for hourly candles. **Not true for daily ones** — a daily stamp is
    /// the date the candle *ends* on. Same field, two meanings, and it is
    /// written down in `docs/worksheets/twelve-data.md` for that reason.
    pub fn opened_at(&self) -> Result<DateTime<Utc>, CandleError> {
        let naive = NaiveDateTime::parse_from_str(&self.datetime, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| CandleError::NotATime(self.datetime.clone()))?;

        Ok(naive.and_utc())
    }

    /// Has this candle finished?
    ///
    /// **Asked of the clock, never of position in the list.**
    ///
    /// The newest candle is usually the one still running — but not always.
    /// Ask at 16:00:02 and you get either the 16:00 candle already open, if a
    /// price has landed, or the 15:00 one now finished, if none has. Counting
    /// from the top would be right most of the time and wrong the rest, which
    /// is worse than being wrong always, because you stop checking.
    pub fn is_finished(&self, now: DateTime<Utc>) -> Result<bool, CandleError> {
        self.finished_by(now, INTERVAL_MINUTES)
    }

    /// The same question, for a candle of any length.
    ///
    /// **A 4-hour candle does not exist until its last hour has closed.** Three
    /// 1-hour candles can close inside a zone and the 4-hour still has nothing
    /// to say; the fourth close is when it does.
    ///
    /// Asking `is_finished` about a 4-hour candle would call it finished three
    /// hours early — and reading a candle before it exists is the one mistake
    /// in this project that makes results look *better* rather than broken.
    pub fn finished_by(&self, now: DateTime<Utc>, minutes: i64) -> Result<bool, CandleError> {
        let one_step = TimeDelta::try_minutes(minutes).ok_or(CandleError::ImpossibleInterval)?;

        Ok(self.opened_at()? + one_step <= now)
    }

    /// How far this candle moved, and which way.
    pub fn change(&self) -> Decimal {
        self.close - self.open
    }

    /// The move as a percentage of where it opened.
    pub fn change_percent(&self) -> Decimal {
        if self.open == Decimal::ZERO {
            return Decimal::ZERO;
        }

        (self.change() / self.open * Decimal::from(100)).round_dp(2)
    }
}

/// How big a normal candle is, over the last `count` of them.
///
/// **True range, not high minus low.** A candle that gapped away from the one
/// before it moved further than its own body shows, and ignoring that makes
/// every band too thin on exactly the days price is moving most.
///
/// `bars` are oldest first.
pub fn normal_candle(bars: &[&Bar], count: usize) -> Option<Decimal> {
    if bars.len() < 2 {
        return None;
    }

    let ranges: Vec<Decimal> = bars
        .windows(2)
        .map(|pair| {
            let (before, now) = (pair[0], pair[1]);

            let high_low = now.high - now.low;
            let gap_up = (now.high - before.close).abs();
            let gap_down = (now.low - before.close).abs();

            high_low.max(gap_up).max(gap_down)
        })
        .collect();

    let recent = &ranges[ranges.len().saturating_sub(count)..];
    if recent.is_empty() {
        return None;
    }

    Some(recent.iter().sum::<Decimal>() / Decimal::from(recent.len()))
}
