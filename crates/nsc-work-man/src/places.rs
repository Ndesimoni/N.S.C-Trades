//! Where things are — his inbox, the settings, and where cards are drawn.
//!
//! **One place, because these were written out by hand wherever they were
//! needed.** His chat id sat in three files and `config/pairs` in five. None
//! of them disagreed, which is the only reason nobody noticed: two copies of
//! the same string agree right up until the day one of them is changed.
//!
//! Moving a settings file then means finding every copy, and the one that gets
//! missed does not fail to compile. It reads a folder that is not there and
//! reports no levels — which looks exactly like a pair he never drew on.

/// Only he may write levels, and everything the bot works out loud goes here.
///
/// **Channel posts carry no sender at all** — Telegram strips it, because a
/// post is from the channel rather than from a person. So the private chat is
/// the only place the bot can tell who is talking.
///
/// It is also where alerts and charts go. **Alerts are not signals**, and
/// mixing the two turns the signal channel into a scratchpad.
pub const OWNER: i64 = 6089491075;

/// One file per pair. **The file is why the pair is watched** — take it out of
/// this folder and the pair stops.
pub const PAIRS: &str = "config/pairs";

/// How thick a band is, how close counts as near, where a graze ends.
pub const THICKNESS: &str = "config/levels.toml";

/// The trading day, the silent days, and when the heartbeat is due.
pub const CALENDAR: &str = "config/when.toml";

/// Where cards are drawn, so the design can be opened in a browser.
pub const PREVIEW: &str = "preview";
