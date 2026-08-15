//! Doing a job again when the trouble says it is worth it.
//!
//! **This is here and not in `nsc-core` because it sleeps.** Waiting is doing;
//! knowing whether a failure is worth another go is knowing. The compiler
//! found the difference the moment the crates were split — `nsc-core` has no
//! `tokio`, so nothing in it can wait for anything.

mod again;

#[cfg(test)]
mod tests;

pub use again::keep_trying;
