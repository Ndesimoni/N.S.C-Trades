//! The background jobs.
//!
//! Each one restarts if it crashes, and each one has to report that it is
//! alive.
//!
//! A job that crashes gets restarted with a delay. A job that stops reporting
//! is treated as dead even if technically it is still running — and that is
//! the case that actually happens.

pub mod feed;
pub mod health;
pub mod news;
pub mod tracker;
