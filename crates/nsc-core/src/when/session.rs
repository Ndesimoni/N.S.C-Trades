//! Which trading day a moment belongs to.
//!
//! **Not the calendar day.** Sunday 22:00 UTC is already in Monday's session,
//! and Monday 22:00 UTC is in Tuesday's.

use chrono::{DateTime, Datelike, Duration, TimeDelta, TimeZone, Utc, Weekday};

use super::Rules;

/// The session this moment belongs to.
///
/// Past the day's close, the next day's session has already begun — which is
/// why Sunday evening is Monday.
pub fn trading_day(now: DateTime<Utc>, rules: &Rules) -> Weekday {
    let local = now.with_timezone(&rules.zone);

    if local.time() >= rules.day_ends {
        local.weekday().succ()
    } else {
        local.weekday()
    }
}

/// How long this session has been running.
///
/// For the settle window — the first hours of a day, where a move gets faked
/// and taken back.
///
/// **Worked out on the New York clock and then converted**, so the answer is
/// right across the two weekends a year when the clocks move and the day is
/// 23 or 25 hours long.
pub fn into_day(now: DateTime<Utc>, rules: &Rules) -> TimeDelta {
    now - opened(now, rules)
}

/// **Are the opening hours over?**
///
/// The first hours of a day are where a move gets faked and taken back. He
/// does not want his phone going during them — a zone touched at the open and
/// abandoned twenty minutes later is noise he would have to ignore, and a
/// buzz he learns to ignore is a buzz that costs him the one that mattered.
///
/// When it turns true he gets one report of where price actually stands, which
/// is the thing he wanted at the open and could not trust yet.
pub fn settled(now: DateTime<Utc>, rules: &Rules) -> bool {
    into_day(now, rules) >= Duration::hours(rules.settle_hours)
}

/// The moment this session opened, as a real instant.
pub fn opened(now: DateTime<Utc>, rules: &Rules) -> DateTime<Utc> {
    let local = now.with_timezone(&rules.zone);

    // Before today's close, this session started at YESTERDAY's close.
    let day = if local.time() >= rules.day_ends {
        local.date_naive()
    } else {
        local.date_naive() - Duration::days(1)
    };

    let wall = day.and_time(rules.day_ends);

    // A wall-clock time can be skipped or repeated on the two days a year the
    // clocks move. `earliest` is the honest pick: on the repeated hour the
    // session opened the FIRST time that clock time came round, and on the
    // skipped one it lands on the instant the hour began.
    rules
        .zone
        .from_local_datetime(&wall)
        .earliest()
        .map(|at| at.with_timezone(&Utc))
        .unwrap_or(now)
}
