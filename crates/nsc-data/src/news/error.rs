//! What can go wrong asking for the calendar.

use nsc_core::error::{Answer, Knows};
use thiserror::Error;

/// Trouble getting the week's file.
#[derive(Debug, Error)]
pub enum CalendarError {
    /// The line failed. Their end, our end, the wifi.
    #[error("could not reach the calendar: {0}")]
    Unreachable(String),

    /// They answered, and it was not a success.
    #[error("the calendar answered {status}")]
    Refused { status: u16 },

    /// **The refusal that arrives looking like a success.**
    ///
    /// Over the download limit, ForexFactory sends an HTML page saying
    /// "Request Denied" with a perfectly normal 200 on it. Nothing about the
    /// status line says no.
    #[error("the calendar refused politely — asked for JSON and got a web page")]
    NotJson,

    /// It was JSON, and it was not a calendar.
    #[error("the calendar's shape changed: {0}")]
    NotEvents(String),
}

impl Knows for CalendarError {
    fn answer(&self) -> Answer {
        match self {
            // A hiccup. Ask again shortly.
            CalendarError::Unreachable(_) => Answer::soon(),

            // **This is the rate limit wearing a disguise**, so it is worth
            // another go — but properly later, not in three seconds. Asking
            // again straight away is what got us refused.
            CalendarError::NotJson => Answer::in_a_while(),

            // Their end is unwell. Wait properly.
            CalendarError::Refused { .. } => Answer::in_a_while(),

            // **The file is not what this code was written against.** Another
            // go returns the same thing. Stop, and let it be said out loud —
            // retrying a broken parser forever looks exactly like a dead line.
            CalendarError::NotEvents(_) => Answer::GiveUp,
        }
    }
}
