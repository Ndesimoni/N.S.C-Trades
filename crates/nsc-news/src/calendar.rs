//! Keeping the economic calendar up to date.
//!
//! Pulls scheduled announcements with the currency they affect and how big
//! they usually are. Safe to run repeatedly — it updates rather than
//! duplicating.
//!
//! Sync ahead of time, not just today. A blackout is only useful if you know
//! about it before the setup forms. Finding out about Non-Farm Payrolls thirty
//! seconds beforehand is not a filter, it is a coincidence.
