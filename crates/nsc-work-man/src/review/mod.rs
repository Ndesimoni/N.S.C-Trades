//! Drawing a pair's levels so he can see where they landed.
//!
//! **This is the check that matters.** Everything in this project is measured
//! against his levels, so the first question is always whether the band we
//! build sits where the one he drew sits.
//!
//! Reading a price back at him only proves he can read his own typing. A
//! picture shows the *place*, which is the thing that actually goes wrong.

mod drawn;
mod picture;

#[cfg(test)]
mod tests;

pub use drawn::Drawn;
pub use picture::picture_of;
