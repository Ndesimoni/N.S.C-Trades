//! Every price that comes down the line.
//!
//! **It sends nothing.** All it does is remember the latest price, so the
//! report made when watching resumes can say where things stand without
//! waiting for the socket to send another.
//!
//! ## Rung 1 was taken out on 1 September 2026
//!
//! There used to be a card here: *price is coming up on your zone*. His call,
//! 1 September 2026:
//! *"when price is getting to a level we do not want an alert, so remove the
//! card. We should only get alerts if the price came from below the band level
//! and closed above it, and vice versa."*
//!
//! It went through three attempts at being quiet enough — once per touch, then
//! once per visit, then once per level ever — and the honest answer was that
//! price reaching a line he drew is not news. **He drew the line. He knows
//! where it is.** What he cannot see without being at the screen is a candle
//! finishing on the other side of it.
//!
//! So the bot has two messages now: a candle that broke a level, and a shape
//! he trades at one.
//!
//! **The price is the middle of the spread**, worked out in
//! `nsc-data::sources::ibkr::ticks` from the last bid and the last ask. It has
//! to be, because the candles come back as mid prices: measured against a bid,
//! a level would look reached when the candle says it never was.

use std::collections::HashMap;

use nsc_data::source::Price;

use super::Watching;

/// **It cannot fail, and the signature says so.** It returned a `Result` while
/// it was still drawing and sending a card. Nothing here reaches anything now,
/// so a caller writing `?` would be handling something that cannot happen.
pub fn heard(watching: &mut HashMap<String, Watching>, heard: Price) {
    // A pair he stopped watching while the line was open. Its subscription
    // outlives the decision by a moment.
    let Some(seen) = watching.get_mut(&heard.symbol) else {
        return;
    };

    seen.watch.saw(heard.mid);
}
