//! The typed version of `config/ta.toml`, with sanity checks on load.
//!
//! Parsing happens elsewhere; this module owns the checking. Catching a
//! nonsensical combination here — say a "close to the level" tolerance wider
//! than the level grouping distance, which would make every price count as
//! being at every level — is far cheaper than finding it in a backtest that
//! has already run for an hour.
