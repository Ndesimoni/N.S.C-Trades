//! What a pair is, and the numbers that go with it.
//!
//! Carries `pip_size` and `digits`, because a stop distance means nothing
//! without them. 20 pips on USDJPY and 20 pips on EURUSD are different
//! numbers of decimal places and different amounts of money.
//!
//! Also holds the two currencies in the pair. The news filter needs them to
//! ask "does this USD announcement affect this pair?", and `nsc-risk` needs
//! them to spot that four different pairs are really one bet on the dollar.
