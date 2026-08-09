//! The watchdog.
//!
//! Tracks the last candle received for each pair and raises the alarm when a
//! feed has been quiet longer than the market can explain.
//!
//! Weekends and holidays have to be handled properly, or the alert fires every
//! Saturday, gets muted, and a muted alert is the same as no alert.
//!
//! Also reports whether the database and Redis are reachable, and answers the
//! `/health` endpoint.
//!
//! The idea behind this job: **silence tells you nothing.** A quiet market and
//! a dead feed look identical from outside, and only this job can tell you
//! which one you are in.
