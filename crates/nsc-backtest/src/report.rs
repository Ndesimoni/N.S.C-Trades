//! Showing the results: terminal summary, a CSV of trades, an equity curve.
//!
//! The breakdowns are worth more than the headline number. Results split by
//! pair, by session, by day of the week, by confidence band, and by which
//! layer rejected the setups that never fired.
//!
//! That last one answers the question you will ask most often — "why did
//! nothing fire this week?" — without rerunning anything.
//!
//! The confidence-band split is the first honest test of your scoring. If
//! high-scoring signals do not do better than low-scoring ones, your scores
//! are not measuring anything.
