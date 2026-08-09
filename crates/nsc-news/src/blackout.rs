//! The actual news block.
//!
//! Answers one question: "is this pair inside a news window right now?"
//!
//! A pair is blocked when a big announcement for **either** of its currencies
//! falls inside the window. US jobs data blocks every dollar pair, not just
//! EURUSD.
//!
//! One thing worth getting right: the window **after** a release should
//! usually be wider than the window before. The initial spike is the smaller
//! problem. The whipsaw for the next twenty minutes is what takes out
//! perfectly good stops.
//!
//! Blackouts apply in backtests too, using the old calendar. A backtest that
//! trades straight through every jobs report is measuring a system you are not
//! going to run.
