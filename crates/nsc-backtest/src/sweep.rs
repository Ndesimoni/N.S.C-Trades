//! Trying lots of settings combinations at once.
//!
//! Run in parallel, because the clean crates make each run completely
//! independent. This is the payoff for keeping the analysis free of databases
//! and internet calls.
//!
//! ## Read the shape, not the winner
//!
//! What comes out is a landscape, not a leaderboard.
//!
//! Look for a **broad patch** where nearby settings all do reasonably well. A
//! single combination that beats its neighbours by a mile has found a quirk of
//! this particular history. Adopting it feels like being thorough and is
//! actually overfitting.
//!
//! Test the fewest settings that answer your question. A grid across six
//! settings will always contain something spectacular, purely because it is
//! enormous.
