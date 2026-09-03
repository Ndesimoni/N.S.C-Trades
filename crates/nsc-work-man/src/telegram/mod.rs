//! Sending to the channel.

mod error;
mod out;

#[cfg(test)]
mod tests;

pub use error::SendError;
pub use out::{ask_words, send, send_to, send_with_buttons, send_words};
