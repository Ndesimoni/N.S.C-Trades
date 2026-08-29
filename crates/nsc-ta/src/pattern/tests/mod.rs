//! What a run of candles is called.
//!
//! ```text
//!     settings.rs  the thresholds he actually runs with
//!     naming.rs    every pattern has its own name, and an arity
//!     runs/        the candle runs, every one of which printed
//!     two.rs       two candles together
//!     three.rs     the star, and the abandoned baby inside it
//!     push.rs      HIS own -- a push, then a pin that gets refused
//! ```

mod naming;
mod push;
mod runs;
mod settings;
mod three;
mod two;

use settings::rules;
