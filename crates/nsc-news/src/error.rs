//! Things that go wrong fetching news.
//!
//! One rule dominates: **a failure to read the page must never turn into an
//! empty calendar.**
//!
//! An empty calendar looks exactly like a quiet news day. If ForexFactory
//! changes its layout and the reader silently returns nothing, your news
//! blocking is switched off and every log line still looks normal. You would
//! find out during a jobs report.
//!
//! So "could not read it" is an error, never an empty list — and if the
//! calendar is unavailable, the blackout check treats that as a reason to be
//! careful, not a reason to carry on.
