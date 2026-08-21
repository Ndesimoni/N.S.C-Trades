//! Is this a pair IBKR will actually serve?
//!
//! **The broker is the only authority on this, and spelling is not.**
//!
//! ```text
//!     AUDUS      wrong length      spelling catches it
//!     AUDUSDD    wrong length      spelling catches it
//!     AUDUSS     right shape       SPELLING CANNOT CATCH IT
//! ```
//!
//! `AUDUSS` is six letters that split neatly into `AUD/USS`, and `USS` is not
//! a currency. It is also exactly the typo a thumb makes. Before this file
//! existed the bot answered *"AUDUSS is new. Which timeframe?"*, wrote the
//! file, saved the level, and then said **"the levels are safe"** — and that
//! pair never reported anything again, with nothing anywhere to say why.

use nsc_core::levels::with_slash;
use nsc_data::sources::ibkr::{IbkrConnection, Serves};

/// What came back about a name he typed.
pub(super) enum Verdict {
    /// IBKR serves it. Carry on.
    Fine,

    /// IBKR answered and has never heard of it. **Do not write anything.**
    Never(String),

    /// IBKR could not be asked at all.
    ///
    /// **Not the same as "no", and it must never be treated as one.** TWS
    /// being shut is an ordinary Tuesday; treating that as "no such pair"
    /// would refuse every real pair he owns.
    CouldNotAsk(String),
}

/// Ask IBKR about a pair name.
///
/// **It opens its own line, one client id along.** The watcher holds the id
/// from `.env` for weeks at a time, and coming in on that id would throw the
/// bot off the feed to answer a question about spelling.
pub(super) async fn pair(name: &str) -> Verdict {
    let symbol = with_slash(name);

    let ibkr = match IbkrConnection::connect_beside().await {
        Ok(ibkr) => ibkr,
        Err(trouble) => return Verdict::CouldNotAsk(format!("{trouble}")),
    };

    match ibkr.serves(&symbol).await {
        Ok(Serves::Yes) => Verdict::Fine,
        Ok(Serves::Never { why }) => Verdict::Never(why),
        Err(trouble) => Verdict::CouldNotAsk(format!("{trouble}")),
    }
}
