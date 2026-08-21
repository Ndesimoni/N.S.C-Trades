//! The heartbeat — one line, only on a day that said nothing else.
//!
//! **It answers one question: is this thing still running?**
//!
//! Silence is the default everywhere else in this bot, which leaves exactly
//! one problem — after a quiet day he cannot tell whether nothing happened or
//! the bot died. On a busy day this never fires at all.
//!
//! It is the only message that goes out on a Monday, when nothing is watched.

use std::collections::HashMap;

use crate::retry::keep_trying;
use crate::{card, telegram};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use nsc_core::when::{Rules, beat_due, beat_words, opened};

use std::path::PathBuf;

use super::Watching;
use crate::places::{OWNER, PREVIEW};

/// Remembers when anything was last said, so the heartbeat knows to stay quiet.
pub struct Pulse {
    /// When the bot last said anything at all — an alert, a close, anything.
    spoke: Option<DateTime<Utc>>,

    /// When the heartbeat itself last went out.
    beat: Option<DateTime<Utc>>,
}

impl Pulse {
    pub fn new() -> Self {
        Pulse {
            spoke: None,
            beat: None,
        }
    }

    /// Call after **anything** is sent. A day that spoke needs no heartbeat.
    pub fn spoke(&mut self, at: DateTime<Utc>) {
        self.spoke = Some(at);
    }

    /// Sends the heartbeat if it is due, and says nothing otherwise.
    pub async fn maybe(
        &mut self,
        client: &reqwest::Client,
        watching: &HashMap<String, Watching>,
        calendar: &Rules,
    ) -> Result<()> {
        let now = Utc::now();

        if !beat_due(now, self.spoke, self.beat, calendar) {
            return Ok(());
        }

        // Sorted, so the list does not shuffle between mornings. A HashMap
        // hands them back in a different order every run, and a card that
        // reorders itself looks like something changed when nothing did.
        let mut seen: Vec<&Watching> = watching.values().collect();
        seen.sort_by(|a, b| a.pair.symbol.cmp(&b.pair.symbol));

        let alive: Vec<card::Alive<'_>> = seen
            .iter()
            .map(|seen| card::Alive {
                pair: &seen.pair,
                bands: seen.watch.bands(),
                price: seen.watch.last_price(),
            })
            .collect();

        let hours = (now - opened(now, calendar)).num_hours();
        let quiet = format!("{hours} hour{}", if hours == 1 { "" } else { "s" });
        let stamp = now.format("%-d %b · %H:%M UTC").to_string();

        let out = PathBuf::from(PREVIEW).join("heartbeat.png");
        let picture = card::heartbeat(&alive, &quiet, &stamp, &out)
            .context("could not draw the heartbeat")?;

        let pairs = watching.len();
        let zones: usize = watching.values().map(|seen| seen.watch.count()).sum();
        println!("Heartbeat — {pairs} pairs, {zones} zones, nothing said today.");

        let owner = OWNER.to_string();
        let pictures = [picture.as_path()];
        let words = beat_words(pairs, zones);

        keep_trying(3, || telegram::send_to(client, &owner, &pictures, &words))
            .await
            .context("could not send the heartbeat")?;

        // **Marked only once it has gone.** Marked first, one failed send
        // silenced the heartbeat for the whole day — and its entire job is
        // telling him the bot is alive on a day nothing else does.
        self.beat = Some(now);

        Ok(())
    }
}
