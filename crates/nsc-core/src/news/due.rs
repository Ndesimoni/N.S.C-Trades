//! Is this worth saying yet — and what belongs on the same card.

use chrono::{DateTime, Duration, Utc};

use super::{Event, Rules};

/// How many minutes until it prints. Negative once it has.
pub fn minutes_until(event: &Event, now: DateTime<Utc>) -> i64 {
    (event.at - now).num_minutes()
}

/// Does this event earn a message right now?
///
/// **A window with two edges, and the far one is the important one.**
///
/// The near edge is obvious: say something `warn_minutes` before. The far edge
/// is what stops a restart being a disaster. Come back up at two in the
/// afternoon and the week's file is full of this morning — without the far
/// edge every one of those is "coming up" and they all arrive at once.
///
/// So: from `warn_minutes` before until `stale_minutes` after, and silence on
/// either side of that.
pub fn due(event: &Event, now: DateTime<Utc>, rules: &Rules) -> bool {
    if !rules.wants(event.impact) {
        return false;
    }

    let opens = event.at - Duration::minutes(rules.warn_minutes);
    let shuts = event.at + Duration::minutes(rules.stale_minutes);

    now >= opens && now <= shuts
}

/// Puts events that print at the same moment onto one card.
///
/// **Three Australian CPI numbers land in the same second.** One card each
/// buzzes his phone three times for what is, to him, a single release — and
/// the whole design rests on messages being rare enough to open.
///
/// Sorted by time first, so the groups come back in the order they will
/// happen rather than the order the feed listed them.
pub fn together<'a>(events: &[&'a Event]) -> Vec<Vec<&'a Event>> {
    let mut sorted = events.to_vec();
    sorted.sort_by_key(|event| event.at);

    let mut groups: Vec<Vec<&'a Event>> = Vec::new();

    for event in sorted {
        match groups.last_mut() {
            Some(group) if group.first().is_some_and(|first| first.at == event.at) => {
                group.push(event)
            }
            _ => groups.push(vec![event]),
        }
    }

    groups
}
