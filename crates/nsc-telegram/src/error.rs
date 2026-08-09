//! Things that go wrong sending and receiving.
//!
//! Split along a line that matters.
//!
//! **Sending** failures get retried with backoff, and whether it went out is
//! recorded. A signal created but never delivered must not look identical to
//! one that was never created.
//!
//! **Button press** failures are not retried. A rejected press is usually
//! invalid or faked, and retrying just processes bad input again.
