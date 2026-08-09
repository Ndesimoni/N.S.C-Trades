//! Scoring — how confident is this setup?
//!
//! The pass-or-fail gates decide **whether** a setup exists. This decides
//! whether it is worth your attention. Below the minimum it is recorded but
//! never sent; above the high mark, the message gets flagged.
//!
//! The point values in `config/strategy.toml` start as guesses. That is fine,
//! and it is the point. Once you have a few hundred judged signals, the Phase
//! 4 analysis tells you which of them the data actually supports.
//!
//! Expect that answer to be uncomfortable. Most traders find that two of their
//! confluences do nearly all the work and the rest are decoration they have
//! been carrying for years. Measuring is the only way to find out.
