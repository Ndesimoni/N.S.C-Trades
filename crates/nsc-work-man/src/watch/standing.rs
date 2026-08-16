//! Where everything stands, so the inbox can be asked.
//!
//! **The watcher holds the live picture** — which bands are sized, where price
//! was last, which zones it is sitting in. The inbox runs beside it and has
//! none of that.
//!
//! So the watcher publishes a copy whenever it changes, and the inbox reads
//! the latest one. Nothing is shared and nothing is locked: the reader gets
//! whatever the last published copy was, which for a question like "is it
//! running and what is close" is exactly right.

use chrono::{DateTime, Utc};
use nsc_core::levels::{Band, Pair};
use rust_decimal::Decimal;
use tokio::sync::watch;

/// One pair, as `/status` needs it.
#[derive(Clone)]
pub struct Standing {
    pub pair: Pair,
    pub bands: Vec<Band>,

    /// Where price was when the last one came down the line. `None` before any
    /// has, and on a day nothing is watched.
    pub price: Option<Decimal>,
}

/// The whole picture at one moment.
#[derive(Clone)]
pub struct Snapshot {
    pub pairs: Vec<Standing>,

    /// When this session opened, so `/status` can say how long it has been
    /// quiet without working the calendar out a second time.
    pub opened: DateTime<Utc>,

    /// **Nothing is being watched today** — the weekend, or a day he has put
    /// in `silent_days`. The line is not open and no price has arrived, so
    /// every distance on the card would be blank. Saying so in a sentence
    /// beats a card full of dashes.
    pub quiet: bool,
}

impl Snapshot {
    pub fn zones(&self) -> usize {
        self.pairs.iter().map(|one| one.bands.len()).sum()
    }
}

/// The watcher's end and the inbox's end of the same picture.
pub fn channel(first: Snapshot) -> (watch::Sender<Snapshot>, watch::Receiver<Snapshot>) {
    watch::channel(first)
}
