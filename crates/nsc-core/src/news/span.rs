//! How much of the calendar he asked to see.

use chrono::{DateTime, Utc};

use super::{Event, Rules};

/// Today, or the rest of the week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Span {
    /// **Everything today, including what has already printed.**
    ///
    /// The point of asking is to see the shape of the day — what is left, and
    /// what he slept through. Showing only what is ahead makes a morning that
    /// has already had its rate decision look like a quiet one.
    Today,

    /// The whole week the file covers, passed releases and all.
    ///
    /// **Both directions, for the same reason `Today` looks backwards.** A
    /// week with its first three days silently missing does not read as a
    /// week — it reads as a quiet one. Every row says which it is, so nothing
    /// has to be guessed from where it sits in the list.
    Week,
}

/// The events he asked for, soonest first.
///
/// **Filtered by impact, exactly like the warnings are.** One setting, so the
/// list he pulls up and the cards that arrive on their own can never disagree
/// about what counts.
pub fn within<'a>(
    events: &'a [Event],
    now: DateTime<Utc>,
    span: Span,
    rules: &Rules,
) -> Vec<&'a Event> {
    let mut wanted: Vec<&Event> = events
        .iter()
        .filter(|event| rules.wants(event.impact) && inside(event, now, span))
        .collect();

    wanted.sort_by_key(|event| event.at);
    wanted
}

/// **The UTC day, not the trading day.**
///
/// `when/` counts a day from 17:00 New York, because that is where the candles
/// break. The calendar is not candles: ForexFactory prints a Tuesday under
/// Tuesday, and a card that disagreed with the site he reads it on would have
/// to be translated every time.
fn inside(event: &Event, now: DateTime<Utc>, span: Span) -> bool {
    match span {
        Span::Today => event.at.date_naive() == now.date_naive(),

        // The file IS the week, so there is nothing to cut off it. Which side
        // of now each one falls is said on the row rather than by leaving it
        // out.
        Span::Week => true,
    }
}

/// Has this one already printed?
///
/// Only ever true on the `Today` list — the week never looks backwards. The
/// card marks these rather than dropping them, because "nothing left today"
/// and "three already gone" are different afternoons.
pub fn printed(event: &Event, now: DateTime<Utc>) -> bool {
    event.at < now
}
