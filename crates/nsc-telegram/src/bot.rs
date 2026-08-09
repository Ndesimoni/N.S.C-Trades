//! Sending signals and receiving button presses.
//!
//! Uses webhooks rather than polling, since Nginx and HTTPS are already part
//! of the setup. The webhook must check Telegram's secret header — an
//! unprotected endpoint lets anyone who finds the URL send fake button presses
//! and poison your training data. That corrupts it silently and you cannot get
//! it back.
//!
//! Failed sends get retried with backoff, and whether it was delivered is
//! recorded. A signal that was created but never arrived must not look
//! identical to a signal that was never created.
