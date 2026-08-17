//! Sending him a picture of a pair.
//!
//! **Two different questions, and they read differently.**
//!
//! One is the reply to having just saved a level — *did that land where I drew
//! it?* The other is him asking to look at a pair, which he can now do without
//! adding anything to make it happen.
//!
//! Reading a price back at him only proves he can read his own typing. A
//! picture shows the PLACE, which is the thing that actually goes wrong — and
//! it is how he reads a chart anyway.

mod asked;
mod landed;
mod sending;

#[cfg(test)]
mod tests;

pub use asked::of_pair;
pub use landed::show;
