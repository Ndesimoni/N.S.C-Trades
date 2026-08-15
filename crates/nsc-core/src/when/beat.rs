//! Whether the heartbeat is due.
//!
//! **One line, and only if nothing else was sent this session.**
//!
//! Silence is the default, which leaves one problem: after a quiet day he
//! cannot tell whether nothing happened or the bot died. This is the answer,
//! and on a busy day it never fires at all.
//!
//! It goes out **before London opens**, because the point is to know the bot
//! is working before the hours he trades. At the end of the day it would be a
//! post-mortem.

use chrono::{DateTime, Duration, Utc};

use super::{Rules, opened};

/// Is the heartbeat due right now?
///
/// - `spoke` — when the bot last said anything at all, of any kind
/// - `beat` — when the heartbeat itself last went out
///
/// Both are handed in. Nothing here reads a clock or remembers anything, so
/// the backtester can ask this about 2019 by passing 2019 in.
pub fn beat_due(
    now: DateTime<Utc>,
    spoke: Option<DateTime<Utc>>,
    beat: Option<DateTime<Utc>>,
    rules: &Rules,
) -> bool {
    let Some(at) = due_at(now, rules) else {
        return false;
    };

    if now < at {
        return false;
    }

    // Once a session. Otherwise every check after 07:00 on a quiet day sends
    // another one, and a heartbeat that repeats is worse than none — he stops
    // reading it, which is the one thing it cannot survive.
    if beat.is_some_and(|when| when >= at) {
        return false;
    }

    // Something was already said this session. He knows it is alive.
    spoke.is_none_or(|when| when < opened(now, rules))
}

/// The moment this session's heartbeat is due.
///
/// **The first `heartbeat_at` at or after the session opened**, not "today at
/// 07:00". The session runs 17:00 New York to 17:00 New York, so it straddles
/// midnight — asking about the calendar day would make the heartbeat due
/// before the session it is reporting on had begun.
fn due_at(now: DateTime<Utc>, rules: &Rules) -> Option<DateTime<Utc>> {
    let open = opened(now, rules);
    let same_day = open.date_naive().and_time(rules.heartbeat_at).and_utc();

    Some(if same_day >= open {
        same_day
    } else {
        same_day + Duration::days(1)
    })
}

/// What the heartbeat says.
///
/// **One line, and it has to answer "is it working" without being read
/// twice.** Here rather than in the binary so it can be looked at without
/// waiting for a quiet day.
pub fn beat_words(pairs: usize, zones: usize) -> String {
    let s = |many: usize| if many == 1 { "" } else { "s" };

    // Built line by line rather than as one long string with `\` continuations.
    // Those carry the source file's own indentation into the message, and it
    // arrives on his phone with everything after the first line pushed right.
    [
        "🫀 <b>Still running.</b>".to_string(),
        String::new(),
        format!(
            "Watching {pairs} pair{} · {zones} zone{}.",
            s(pairs),
            s(zones)
        ),
        "Quiet since the open — nothing has reached a level.".to_string(),
    ]
    .join("\n")
}
