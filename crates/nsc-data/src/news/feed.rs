//! Asking for the week's file, and checking the answer really is one.

use super::{CalendarError, Parsed, parse};

/// Downloads the calendar and reads it.
///
/// **Nothing here decides anything.** It gives back every event in the file,
/// of every rating. Which of them are worth a message is
/// `nsc_core::news::due`, and keeping that judgement out of the fetching is
/// what lets it be tested without a network.
pub async fn fetch(client: &reqwest::Client, url: &str) -> Result<Parsed, CalendarError> {
    let answer = client
        .get(url)
        .send()
        .await
        .map_err(|trouble| CalendarError::Unreachable(quietly(trouble)))?;

    let status = answer.status();

    if !status.is_success() {
        return Err(CalendarError::Refused {
            status: status.as_u16(),
        });
    }

    let body = answer
        .text()
        .await
        .map_err(|trouble| CalendarError::Unreachable(quietly(trouble)))?;

    if !looks_like_json(&body) {
        return Err(CalendarError::NotJson);
    }

    parse::read(&body)
}

/// Is this the calendar, or the refusal that looks like a success?
///
/// **Over the download limit ForexFactory sends a web page**, not an error —
/// `<!DOCTYPE html>` with "Request Denied" in it, under a perfectly ordinary
/// 200. Handed straight to the JSON parser it comes back as a parse failure,
/// which reads as "the feed changed shape" and would stop the watcher for
/// good over something that clears itself in five minutes.
///
/// **This is the third time this project has met the same trap.** Twelve Data
/// refused with a polite `{"code": 401}`. Telegram refuses with a polite
/// `ok: false`. A reply that parses is not a reply that worked, and here a
/// reply that arrives is not a reply that answered.
///
/// The test is deliberately the weakest one that separates them: a JSON array
/// starts with `[`, a web page starts with `<`. Anything more clever is more
/// to be wrong about.
pub(super) fn looks_like_json(body: &str) -> bool {
    body.trim_start().starts_with('[')
}

/// A network failure with the address taken off it.
///
/// This calendar needs no key, so nothing secret is in this URL today. It is
/// stripped anyway, because the rule "never print the url" was written down
/// once for the feed's key, followed on the happy path, and not applied to the
/// error path — which is the one that prints.
fn quietly(trouble: reqwest::Error) -> String {
    trouble.without_url().to_string()
}
