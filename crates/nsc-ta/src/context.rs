//! What kind of market is this right now?
//!
//! Not signals — background information that changes what a signal means. The
//! same engulfing candle at the same level means different things in a quiet
//! Asian range and in a wild post-news expansion. The Phase 4 model cannot
//! learn that difference unless the difference gets recorded.
//!
//! Provides: how volatile things are compared to recent history, which session
//! it is, where price sits in the day's and week's range, and how far price
//! has already moved today.
