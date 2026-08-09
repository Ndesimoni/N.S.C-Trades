//! Getting an AI to turn a headline into structured information.
//!
//! Headline in; which currencies it affects, which direction, and how serious,
//! out.
//!
//! This is the one job in the whole system where an AI is clearly better than
//! rules. Written English with no fixed format is exactly what it is for.
//!
//! Results get saved on the headline. Re-reading the same text every candle
//! wastes money and adds delay for no new information.
//!
//! The answer must come back in a fixed shape and gets checked on arrival. If
//! this returned free text, something downstream would end up trying to parse
//! it with string matching, which is worse than not having it at all.
