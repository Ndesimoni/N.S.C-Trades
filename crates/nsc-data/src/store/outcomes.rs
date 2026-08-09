//! Saving what the market did after each signal.
//!
//! Records the result as a multiple of what you risked, plus how far price ran
//! in your favour and how far it ran against you before resolving. Those last
//! two answer the questions you will actually have: are my stops too tight,
//! are my targets too greedy?
//!
//! `ambiguous` is a real result, not a bug. When price hit both the stop and
//! the target inside one candle, the data genuinely cannot say which came
//! first. Those get left out of the numbers rather than guessed at — guessing
//! in your own favour is one of the classic ways a backtest flatters itself.
