//! Replaying candles from a file. No internet, same answer every time.
//!
//! This is the workhorse for building and testing, not a fallback. Input that
//! never changes is what makes the chart-reading tests worth anything: the
//! same file must always produce the same swings, the same levels and the same
//! signals, forever. A test fed by a live connection cannot promise that.
