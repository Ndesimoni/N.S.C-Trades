//! Tests that need a real Postgres.
//!
//! ```text
//!     docker compose up -d
//!     cargo test -p nsc-data -- --ignored
//! ```
//!
//! ## Why every one of them is `#[ignore]`
//!
//! The queries here are checked at RUNTIME, not by the compiler — that was the
//! trade for not making `cargo check` need a database. So a typo in SQL is
//! found on the first call, and these are the first call.
//!
//! They cannot run in the ordinary suite, because the ordinary suite has to
//! pass on a machine with no container.
//!
//! **And they must not quietly pass when there is no database.** A test that
//! skips itself and reports green pins nothing at all — which is the thing
//! `CLAUDE.md` says to check for. `#[ignore]` says "not run" out loud;
//! skipping inside the body would say "passed".

mod calendar;
mod candles;
mod deciding;
mod support;
