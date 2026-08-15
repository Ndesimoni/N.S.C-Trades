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

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use nsc_core::when::{Rules, beat_due, beat_words};
use nsc_work_man::telegram;

use super::{OWNER, Watching};

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

        self.beat = Some(now);

        let pairs = watching.len();
        let zones: usize = watching.values().map(|seen| seen.watch.count()).sum();

        let words = beat_words(pairs, zones);

        println!("heartbeat: {pairs} pairs, {zones} zones, nothing said");

        telegram::send_words(client, &OWNER.to_string(), &words)
            .await
            .context("could not send the heartbeat")
    }
}
