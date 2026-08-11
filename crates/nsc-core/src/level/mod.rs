//! Support and resistance as bands of price, not exact numbers.
//!
//! A level is a strip of price that has turned the market before, built from
//! swing points that sit close together. It carries the plain facts about
//! itself: how thick it is, which timeframe it was found on, how many times
//! price has touched it, when it was first and last touched, and the first
//! moment you could have known about it.
//!
//! Bands rather than exact prices, for two reasons. Price does not turn at an
//! exact number — it turns somewhere in a small area. And an exact price
//! forces a tolerance into every comparison you ever write against it, which
//! is the same fixed-pip trap that stops working the moment you add a second
//! instrument.
//!
//! ## One type, not a Support and a Resistance
//!
//! When a support breaks and later holds price down, it is the same level
//! doing a different job. Which side price is on is just where price happens
//! to be today, so it is not part of what a level *is*.
//!
//! ## What this type does not have
//!
//! No `strength` score. No `exhausted` flag. Nothing that says whether the
//! level will hold or break.
//!
//! That is a judgement, and it needs far more than the level itself — the
//! trend, the timeframe, the candle printing into it, how price arrived. It
//! belongs in `nsc-strategy`, driven by `config/strategy.toml`. This type
//! reports what is on the chart and stops there.
//!
//! Read `README.txt` for why that split is worth keeping.
//!
//! ## What is where
//!
//! - [`band`] — `Band`, the strip of price itself
//! - [`zone`] — `Level`, that band plus what is known about it

mod band;
mod zone;

#[cfg(test)]
mod tests;

pub use band::Band;
pub use zone::Level;
