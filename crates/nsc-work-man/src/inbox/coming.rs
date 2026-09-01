//! `/news` — what is on the calendar, today or for the rest of the week.
//!
//! **The warnings that arrive on their own are a different thing.** Those are
//! one release, five minutes ahead and again a minute ahead, and they come
//! whether he asked or not.
//! This is the list, whenever he wants it.
//!
//! Both read the same `config/news.toml`, so what counts as worth showing can
//! never disagree between them.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::news::{self, Event, Span, within};
use nsc_data::news::fetch;
use serde_json::json;

use super::talking::say;
use super::words::{TODAY, WEEK};
use crate::places::{NEWS, OWNER, PREVIEW};
use crate::{card, telegram};

/// Asks which he wants.
pub async fn ask(client: &reqwest::Client, token: &str) -> Result<()> {
    say(
        client,
        token,
        "<b>What is coming up</b>\n\nToday shows the whole day, including what \
         has already printed. This week shows what is left of it.",
        Some(json!([[TODAY, WEEK]])),
    )
    .await
}

/// Fetches the calendar and sends the list.
///
/// **It reaches the network, so it can take a moment**, and drawing takes
/// Chrome a few seconds on top. He tapped a button and is looking at the
/// screen, so it says something first.
pub async fn show(client: &reqwest::Client, token: &str, span: Span) -> Result<()> {
    let rules = news::load(Path::new(NEWS)).with_context(|| format!("could not read {NEWS}"))?;

    let parsed = fetch(client, &rules.url)
        .await
        .context("could not download the economic calendar")?;

    if parsed.unreadable > 0 {
        eprintln!(
            "The calendar had {} row{} whose time made no sense.",
            parsed.unreadable,
            if parsed.unreadable == 1 { "" } else { "s" }
        );
    }

    let now = Utc::now();
    let wanted = within(&parsed.events, now, span, &rules);

    // **A quiet day gets words and no picture.** Running Chrome for the best
    // part of ten seconds to draw one line saying "nothing" is the same
    // mistake /status made on a resting day.
    if wanted.is_empty() {
        return say(client, token, &nothing(span), None).await;
    }

    let out = PathBuf::from(PREVIEW).join("calendar.png");
    let words = caption(&wanted, span);

    // **It answers either way.** He asked a question outright, so replying
    // "could not do that" to one the bot can in fact answer is the worst of
    // both. The picture carries the answer better; the words carry it.
    //
    // Drawing and sending are both covered, because a photo Telegram refuses
    // leaves him just as unanswered as one Chrome never drew. `/status`
    // learned this the same way.
    // **Off the inbox loop**, same as `/status`. Two to ten seconds of Chrome
    // is two to ten seconds of not answering him.
    let owned: Vec<Event> = wanted.iter().map(|&event| event.clone()).collect();

    let drawn = tokio::task::spawn_blocking(move || {
        let borrowed: Vec<&Event> = owned.iter().collect();
        card::calendar(&borrowed, span, now, &out)
    })
    .await?;

    let sent = match drawn {
        Ok(picture) => telegram::send_to(client, &OWNER.to_string(), &[&picture], &words)
            .await
            .map_err(anyhow::Error::from),

        Err(trouble) => Err(trouble.into()),
    };

    if let Err(trouble) = sent {
        eprintln!("Could not send the calendar card: {trouble:#}");
        return say(client, token, &words, None).await;
    }

    Ok(())
}

/// What to say when nothing clears the bar.
///
/// **Silence would be wrong here.** He asked outright, and no answer is
/// indistinguishable from the bot being dead.
fn nothing(span: Span) -> String {
    let when = if span == Span::Today {
        "today"
    } else {
        "for the rest of this week"
    };

    format!(
        "😌 <b>Nothing {when}</b>\n\n\
         No high or medium impact releases on the calendar.\n\n\
         <i>Low impact is left out — it is most of the file and it moves \
         nothing.</i>"
    )
}

/// The line under the picture.
///
/// **It stands on its own**, because a card that will not load leaves only
/// this — and it is also what he gets when Chrome fails.
fn caption(events: &[&Event], span: Span) -> String {
    let when = if span == Span::Today {
        "Today"
    } else {
        "The rest of the week"
    };

    let high = events
        .iter()
        .filter(|event| event.impact == nsc_core::news::Impact::High)
        .count();

    let heavy = if high == 0 {
        String::new()
    } else {
        format!(" · <b>{high}</b> high impact")
    };

    format!(
        "<b>{when}</b> — {} release{}{heavy}",
        events.len(),
        if events.len() == 1 { "" } else { "s" }
    )
}
