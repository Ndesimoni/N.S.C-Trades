//! Runs the six layers in order and produces a setup.
//!
//! ```text
//!   TaSnapshot
//!       ↓
//!   direction  → none?   stop here.
//!       ↓
//!   place      → fails?  stop here.
//!       ↓
//!   trigger    → fails?  stop here.
//!       ↓
//!   stop level  → the stop loss
//!   target      → the take profit and risk-to-reward
//!                 → too small? stop here.
//!       ↓
//!   score       → below minimum? record it, do not send.
//!       ↓
//!   skip checks → blocked? record it with the reason.
//!       ↓
//!   a setup
//! ```
//!
//! Rejections get **recorded**, not thrown away — along with which layer
//! rejected them. Two payoffs. You can answer "why did nothing fire on GBPUSD
//! today?" without rerunning anything. And those rejections become the "don't
//! take this one" examples the Phase 4 model trains on.
