//! Things that can go wrong applying the rules.
//!
//! The important distinction: a settings file that **fails to load** is fatal,
//! but a candle that produced no setup is not an error at all. That is the
//! normal case, and by far the most common outcome.
//!
//! Getting that line wrong is how a bot ends up logging thousands of "errors"
//! a day that are just quiet markets. Then nobody reads the logs, and a real
//! problem goes unnoticed.
