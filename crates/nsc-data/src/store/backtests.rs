//! Saving backtest runs.
//!
//! Stores the full settings snapshot and the code version alongside the
//! results.
//!
//! That is not paperwork. A backtest number means nothing without knowing
//! which version of the chart-reading code produced it. Comparing two runs
//! across a change to swing detection is comparing two different systems — and
//! doing that by accident is how a "promising" setting gets adopted on the
//! strength of a bug.
