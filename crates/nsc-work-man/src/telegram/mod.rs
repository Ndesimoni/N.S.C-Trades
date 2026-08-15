//! Sending to the channel.

mod error;
mod out;

#[cfg(test)]
mod tests;

pub use error::SendError;
pub use out::{send, send_to};
