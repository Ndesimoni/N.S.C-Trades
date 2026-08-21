//! What the buttons say.
//!
//! **Buttons are not set up anywhere.** The bot sends them with a message, and
//! tapping one sends that word back as an ordinary message. A button is a
//! shortcut for typing, nothing more — so the word on the button and the word
//! the bot matches on have to be the same string, and that is why they are all
//! in one file.

use nsc_core::levels::Timeframe;

/// His three charts, and what each is called on a button.
pub(super) const TIMEFRAMES: [(&str, Timeframe); 3] = [
    ("Weekly", Timeframe::Weekly),
    ("Daily", Timeframe::Daily),
    ("4-hour", Timeframe::H4),
];

/// Starting a pair the bot has never seen.
pub(super) const NEW_PAIR: &str = "+ new pair";

/// What he can do to one pair, from its own page.
pub(super) const ADD: &str = "+ Add levels";
pub(super) const DROP: &str = "− Take one off";
pub(super) const CHART: &str = "📈 Chart";
pub(super) const STOP: &str = "✗ Stop watching";

/// Backing out.
///
/// **Every keyboard carries it.** Without one the only ways out of a flow are
/// finishing it or sending a command that happens to replace the buttons —
/// and the buttons stay on his screen in the meantime, over his own keyboard,
/// looking like the bot is waiting for something.
pub(super) const CLOSE: &str = "✗ Close";
pub(super) const UNDO: &str = "↩ Undo";

/// Stopping a pair takes two taps, not one.
///
/// It throws away every level he has drawn for that pair — months of chart
/// work — and it is done by tapping a button on a phone while doing something
/// else.
pub(super) const YES: &str = "✓ Yes, stop it";
pub(super) const NO: &str = "✗ Keep it";
