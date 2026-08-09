//! Things that go wrong talking to brokers and the database.
//!
//! The different kinds exist to answer one question where they are caught:
//! **retry, or give up?**
//!
//!   - timeout or connection dropped → retry, backing off. The broker is
//!     having a moment.
//!   - rate limited                  → retry, but slower, and respect what
//!     the provider told you.
//!   - bad API key                   → **give up.** Retrying a dead key
//!     forever looks exactly like a dead feed and wastes hours of hunting.
//!   - unexpected response format    → **give up and shout.** The provider
//!     changed something, and guessing at the new format corrupts your
//!     history.
//!   - missing candles               → **give up.** Analysis on holed data is
//!     worse than no analysis, because it still produces confident numbers.
//!
//! Lumping these together is how a bot ends up retrying an expired key every
//! thirty seconds for a week.
