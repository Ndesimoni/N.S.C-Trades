//! Limits on how many signals can be open at once — total, per pair, per group.
//!
//! In a signals-only system, the real cost of unlimited signals is your
//! attention. A bot sending fifteen setups a day is a bot you stop reading.
//! And a bot you stop reading is worse than no bot, because you have lost the
//! feedback loop the whole training plan depends on.
