//! The chart replay tool for judging old setups. Phase 4.
//!
//! Shows historical setups one at a time — the chart, the bot's reasoning,
//! and **no result** — and records what you would have done.
//!
//! **The result must stay hidden.** Once you know how it turned out, you are
//! not judging any more, you are remembering. A model trained on that has
//! learned to predict the past.
//!
//! This is how you build up a dataset without waiting months for live signals.
//! Around 200–300 judged setups is where the Phase 4 model becomes worth
//! training. Below that it learns noise and reports confidence while doing so.
