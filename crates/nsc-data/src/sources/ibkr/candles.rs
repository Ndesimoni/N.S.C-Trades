//! Asking IBKR for candles, and turning them into ours.
//!
//! **Two things are wrong with an IBKR candle until this file fixes them.**
//!
//! Its stamp is in whatever timezone TWS was logged in with — the library
//! says so plainly, and his is Dubai. Left alone, every candle would be four
//! hours out and nothing anywhere would error.
//!
//! Its prices are `f64`, and this project never keeps a price that way. The
//! rounding below is where a float stops being one.

use chrono::DateTime;
use ibapi::market_data::TradingHours;
use ibapi::market_data::historical::{BarSize, Duration, ToDuration, WhatToShow};
use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use crate::source::{Interval, MarketDataSource};

use super::connect::IbkrConnection;
use super::contract;
use super::error::IbkrError;

/// How the stamp is written, to match what `Bar::opened_at` parses.
const STAMP: &str = "%Y-%m-%d %H:%M:%S";

/// How many decimal places survive the trip out of `f64`.
///
/// Eight is past anything quoted — spot forex is five, gold is two. It is
/// here to shave off float dust, not to round a price.
const DECIMALS: u32 = 8;

impl MarketDataSource for IbkrConnection {
    type Trouble = IbkrError;

    async fn candles(
        &self,
        symbol: &str,
        interval: Interval,
        count: usize,
    ) -> Result<Vec<Bar>, IbkrError> {
        let contract = contract::for_symbol(symbol)?;

        let answer = self
            .client
            .historical_data(
                &contract,
                // None means "up to now".
                None,
                span(interval, count),
                bar_size(interval),
                // **Spot forex has no TRADES.** There is no central exchange
                // to trade on, so asking for them is refused. MidPoint is both
                // the only sensible ask and the one the chart-reading wants.
                Some(WhatToShow::MidPoint),
                // **Extended, not Regular.** Regular hours would cut the day
                // down to an exchange session that spot forex does not have,
                // and the candles would stop matching his chart.
                TradingHours::Extended,
            )
            .await
            .map_err(|e| IbkrError::Refused {
                symbol: symbol.to_string(),
                why: e.to_string(),
            })?;

        // **IBKR sends oldest first. Everything here expects newest first.**
        // Getting this backwards does not error — it sizes bands off the
        // oldest candles on file and reports a candle from last year as the
        // one that just closed.
        let mut bars: Vec<Bar> = answer.bars.iter().map(into_bar).collect::<Result<_, _>>()?;

        bars.reverse();
        bars.truncate(count);

        Ok(bars)
    }
}

/// One IBKR candle as one of ours.
///
/// **The stamp goes through an absolute instant on purpose.** `unix_timestamp`
/// is the same number whatever timezone TWS reports in, so the conversion is
/// right without anybody having to know what that timezone was.
fn into_bar(raw: &ibapi::market_data::historical::Bar) -> Result<Bar, IbkrError> {
    Ok(Bar {
        datetime: stamp(raw.date.unix_timestamp())?,
        open: price("open", raw.open)?,
        high: price("high", raw.high)?,
        low: price("low", raw.low)?,
        close: price("close", raw.close)?,
    })
}

/// A candle's start time, in UTC, written the way `Bar::opened_at` reads it.
///
/// **This is the whole timezone fix, and it is one line.** Seconds since the
/// epoch are the same number in Dubai as they are in London, so nothing here
/// needs to know what TWS was set to. Reading the offset off the stamp and
/// trusting it would work perfectly until the day he logs in from somewhere
/// else.
pub(super) fn stamp(seconds: i64) -> Result<String, IbkrError> {
    let when = DateTime::from_timestamp(seconds, 0).ok_or_else(|| {
        IbkrError::NotACandle(format!("its time was {seconds}, which is not one"))
    })?;

    Ok(when.format(STAMP).to_string())
}

/// One price out of a float, and never back into one.
pub(super) fn price(which: &str, value: f64) -> Result<Decimal, IbkrError> {
    Decimal::try_from(value)
        .map(|exact| exact.round_dp(DECIMALS).normalize())
        .map_err(|_| IbkrError::NotACandle(format!("its {which} was {value}")))
}

/// The interval, as IBKR names it.
pub(super) fn bar_size(interval: Interval) -> BarSize {
    match interval {
        Interval::Min5 => BarSize::Min5,
        Interval::Min15 => BarSize::Min15,
        Interval::Min30 => BarSize::Min30,
        Interval::H1 => BarSize::Hour,
        Interval::H4 => BarSize::Hour4,
        Interval::Day => BarSize::Day,
        Interval::Week => BarSize::Week,
    }
}

/// How far back to ask, to be sure of getting `count` candles.
///
/// **Always more than the arithmetic says.** The market is shut at weekends,
/// so sixty daily candles are not sixty days ago — they are twelve weeks ago.
/// Asking for exactly enough returns too few, and too few silently makes a
/// "normal candle" out of a shorter sample than the one that was asked for.
pub(super) fn span(interval: Interval, count: usize) -> Duration {
    let count = count.max(1) as i32;

    match interval {
        // Weekly candles: one a week, plus a month of room.
        Interval::Week => in_weeks(count + 4),

        // Daily candles: five a week, so seven days buys five. Plus a fortnight
        // for holidays.
        Interval::Day => in_days(count * 7 / 5 + 14),

        // Everything intraday, in whole days, doubled for the weekend and
        // never less than one.
        _ => {
            let minutes_wanted = count as i64 * interval.minutes();
            let days = (minutes_wanted / (60 * 24) + 1) * 2;

            in_days(days as i32)
        }
    }
}

/// Weeks, or years once there are too many weeks to say.
///
/// **IBKR refuses more than 52 weeks written as weeks.** It answers
/// *"durations longer than 52 weeks must be made in years"* and serves
/// nothing — which is how sixty weekly candles, the number the band sizing
/// asks for, failed on the first real run.
fn in_weeks(weeks: i32) -> Duration {
    if weeks <= 52 {
        return weeks.weeks();
    }

    in_years(weeks * 7)
}

/// Days, or years once there are too many days to say.
///
/// **The same rule again at 365.** A day count over a year has to be written
/// as years, and the refusal looks identical.
fn in_days(days: i32) -> Duration {
    if days <= 365 {
        return days.days();
    }

    in_years(days)
}

/// Whole years, **rounded up**, never less than one.
///
/// Rounded up because rounding down asks for less history than was wanted,
/// and too few candles does not error — it quietly averages a "normal candle"
/// over a shorter run than the one that was asked for, which changes every
/// band width on the pair.
fn in_years(days: i32) -> Duration {
    ((days + 364) / 365).max(1).years()
}
