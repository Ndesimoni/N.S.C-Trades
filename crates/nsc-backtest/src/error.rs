//! Things that go wrong during a backtest.
//!
//! The one that matters is "used data from the future", raised by `guards.rs`.
//! It kills the run completely and on purpose: a contaminated run must produce
//! **no number at all**, rather than a number with a warning attached.
//!
//! The reasoning is about behaviour, not correctness. A warned-about number
//! still gets read, compared, and eventually acted on — especially when it is
//! a good number. Refusing to produce one is the only reliable way to stop a
//! poisoned backtest influencing a decision six weeks later.
