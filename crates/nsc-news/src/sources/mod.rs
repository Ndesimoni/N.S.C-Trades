//! Where the calendar and headlines come from.
//!
//! `finnhub`      — calendar and headlines from one API key. Simplest start.
//!
//! `forexfactory` — the calendar most retail forex traders actually read.
//!                  Read off the web page, so it breaks whenever the page
//!                  changes. It must fail **loudly** rather than quietly
//!                  returning nothing — an empty calendar looks exactly like a
//!                  quiet news day, so a silent failure would switch off your
//!                  news blocking with no error anywhere.

pub mod finnhub;
pub mod forexfactory;
