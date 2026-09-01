//! Is this worth saying yet — and what belongs on the same card.

use chrono::{DateTime, Duration, Utc};

use super::{Event, Rules};

/// How many minutes until it prints. Negative once it has.
pub fn minutes_until(event: &Event, now: DateTime<Utc>) -> i64 {
    (event.at - now).num_minutes()
}

/// **Which warning is live for this event right now**, in minutes before it —
/// or nothing, if none is.
///
/// `warn_at_minutes = [30, 5]` gives a heads-up half an hour out and a last
/// call five minutes out. His ask, 1 September 2026.
///
/// ## One mark at a time, and that is what makes two cards work
///
/// Each mark owns the stretch from where it opens until the next one takes
/// over. The last one runs on past the event by `stale_minutes`:
///
/// ```text
///     30  ├──────────────────────┤              at-30 up to at-5
///      5                         ├────────┤     at-5 through at+5
///                                    ↑
///                                the event
/// ```
///
/// **They never overlap**, so "which card is this" always has one answer, and
/// the caller can remember having sent each one separately. Windows that both
/// stayed open would make the second card either impossible to tell from the
/// first or impossible to send at all.
///
/// **THE FAR EDGE IS THE IMPORTANT ONE**, and it belongs to the last mark.
/// Come back up at two in the afternoon and the week's file is full of this
/// morning — without it every one of those is "coming up" and they all arrive
/// at once. A restart just after a release finds only the last mark live, and
/// that is the right card: five minutes to it is the news, half an hour ago is
/// history.
pub fn due_at(event: &Event, now: DateTime<Utc>, rules: &Rules) -> Option<i64> {
    if !rules.wants(event.impact) {
        return None;
    }

    // **Tidied here rather than trusted from the file.** A trader writing
    // `[5, 30]` or `[30, 30, 5]` means the obvious thing, and the windows
    // below only line up if the marks run widest first with no repeats.
    let mut marks: Vec<i64> = rules.warn_at_minutes.clone();
    marks.sort_unstable();
    marks.dedup();
    marks.reverse();

    for (which, mark) in marks.iter().enumerate() {
        let opens = event.at - Duration::minutes(*mark);

        match marks.get(which + 1) {
            // A narrower mark takes over where this one ends.
            Some(next) => {
                if now >= opens && now < event.at - Duration::minutes(*next) {
                    return Some(*mark);
                }
            }

            // The last one, and the only one that outlives the event.
            None => {
                if now >= opens && now <= event.at + Duration::minutes(rules.stale_minutes) {
                    return Some(*mark);
                }
            }
        }
    }

    None
}

/// Does this event earn a message right now? For anything that does not care
/// **which** warning it is.
pub fn due(event: &Event, now: DateTime<Utc>, rules: &Rules) -> bool {
    due_at(event, now, rules).is_some()
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
