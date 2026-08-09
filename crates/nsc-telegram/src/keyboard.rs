//! The buttons under each signal.
//!
//! Every signal gets 👍 "would take" and 👎 "would skip". Two taps of effort,
//! and it is the entire way training data gets collected.
//!
//! A third button, "why?", opens a short list of skip reasons: wrong
//! direction, bad level, weak trigger, too far extended, not feeling this pair.
//!
//! Those reasons are where missing rules come from. A skip reason you find
//! yourself picking over and over is a rule that belongs in
//! `config/strategy.toml`.
//!
//! What comes back from a button press has to be checked on the server.
//! Anything arriving from a button is untrusted.
