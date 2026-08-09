//! Layer 6 — reasons to skip a setup that ticked every other box.
//!
//! Sessions you avoid, the Friday cutoff, a cooldown so the same idea does not
//! fire again immediately, spread limits.
//!
//! **This layer is where your real edge probably lives, and it is almost never
//! written down.** When Phase 3 shows you rejecting signals the bot was happy
//! with, the missing rule usually belongs here. Expect this file and the
//! `[veto]` section of `config/strategy.toml` to grow more than anything else.
//!
//! The news blackout is handled in `nsc-news` instead of here, because it
//! needs to reach the outside world and this crate is not allowed to. The
//! answer arrives as an input.
