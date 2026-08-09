//! Keeping the calendar and headlines up to date.
//!
//! Fetches ahead of time, because a news block is only useful if you know
//! about it before the setup forms.
//!
//! Runs on its own schedule rather than on demand, so a slow news provider can
//! never hold up the signal pipeline.
