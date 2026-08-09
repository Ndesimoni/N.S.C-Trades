//! Drawing Fibonacci levels automatically.
//!
//! The ratios are trivial. **Picking which move to measure is the actual
//! work**, and it is where this module earns its keep — the same ratios drawn
//! from a different pair of swings give completely different prices.
//!
//! The rule: measure the most recent confirmed move in the direction of the
//! trend, big enough to matter, so that small noise moves are not used as
//! anchors.
//!
//! The move it picked is returned with the levels and shown in the signal's
//! reasoning. When a Fibonacci signal looks wrong to you, the move it chose is
//! nearly always the disagreement — and that feedback is how this gets tuned.
