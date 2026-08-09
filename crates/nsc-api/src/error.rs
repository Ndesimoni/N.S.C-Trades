//! `ApiError` — the error type for the web endpoints.
//!
//! This is the only error type that turns into HTTP status codes, which is why
//! it lives separately from the library ones.
//!
//! It also draws a security line. Internal errors — database messages,
//! connection strings, stack traces — get **written to the log in full** and
//! returned to the caller as a bare 500 with a reference number. This endpoint
//! is reachable from the open internet, and a leaked database error describes
//! your tables to whoever asked.
//!
//! Rejected button presses return OK, not an error code. Telegram retries
//! anything that is not a success, so a faked press answered with an error
//! turns into a retry loop against your own server.
