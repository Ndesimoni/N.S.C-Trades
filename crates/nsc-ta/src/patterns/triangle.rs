//! Ascending, descending and symmetrical triangles.
//!
//! Built from two trendlines closing in on each other, so this leans on
//! `trendlines.rs` instead of redoing the line fitting.
//!
//! How close price is to the point where the lines meet matters. A triangle is
//! tradeable partway through and unreliable once price has crawled into the
//! tip, where breakouts fail most often. That distance gets reported so the
//! rules can refuse late ones.
