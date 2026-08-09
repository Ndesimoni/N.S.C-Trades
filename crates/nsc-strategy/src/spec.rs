//! Reading `config/strategy.toml`, and checking it makes sense.
//!
//! Loading is not just reading the file. This module rejects settings that
//! contradict each other — a minimum risk-to-reward the chosen target method
//! can never reach, a "place" layer needing more confluences than it has
//! sources switched on, a trigger timeframe that is not in the list of
//! timeframes the bot watches.
//!
//! Every one of those shows up as "the bot never sends anything", which is a
//! miserable thing to debug from the outside.
//!
//! `version` gets bumped whenever the rules change, so old backtest results
//! stay comparable. Comparing results across an unversioned rule change means
//! comparing two different systems while thinking you are measuring one.
