//! Optional second opinion on the chart picture.
//!
//! Sends the same image that goes to Telegram and asks whether the setup looks
//! valid.
//!
//! Genuinely useful, because it looks at the chart the way you do — visually,
//! in context — rather than through a list of numbers. So it can catch things
//! the numbers do not capture: a level drawn through obvious chop, a "clean"
//! approach that clearly is not.
//!
//! Same limits as everywhere in this crate: advice only, never able to approve
//! a rejected setup, and never asked to read prices off the picture. It will
//! read them wrong.
