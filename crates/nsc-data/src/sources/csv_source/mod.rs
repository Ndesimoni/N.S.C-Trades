//! Reading candles from a file. No internet, same answer every time.
//!
//! This is the workhorse for building and testing, not a fallback. Input that
//! never changes is what makes the chart-reading tests worth anything: the
//! same file must always produce the same swings, the same levels and the same
//! signals, forever. A test fed by a live connection cannot promise that.
//!
//! ## What it accepts
//!
//! A CSV with a header row. Column names are matched case-insensitively and
//! the common exports are understood:
//!
//! ```text
//!   time · timestamp · date · datetime · open_time     when it STARTED
//!   open · high · low · close                          the four prices
//!   volume                                             optional, ignored
//! ```
//!
//! Order does not matter — the header says which column is which. What is
//! **not** guessed at is a file with no header, because then the order is all
//! there is, and a file whose columns are arranged differently would parse
//! perfectly and be wrong in every price.
//!
//! ## Times are read as UTC
//!
//! Whatever the file says, the times are taken as UTC unless they carry an
//! offset of their own. Guessing a timezone would shift every candle, and with
//! it every level the bot draws.
//!
//! If your export is in broker time, convert it before it gets here. That is
//! one job done once, rather than a guess repeated on every row.
//!
//! ## One thing to watch, and it cannot be checked here
//!
//! **The last row of a file exported mid-session is a candle still forming.**
//! Its high and low have not finished happening.
//!
//! Nothing in the file says so, and this reader will not pretend to know. Read
//! `README.txt` before trusting the newest candle in a fresh export.
//!
//! ## What is where
//!
//! - [`columns`] — working out which column is which, from the header
//! - [`rows`] — turning one row into one candle
//! - [`read`] — the whole file

mod columns;
mod read;
mod rows;

#[cfg(test)]
mod tests;

pub use read::read_candles;
