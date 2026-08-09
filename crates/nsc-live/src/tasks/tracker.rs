//! Following signals to their result — Phase 3, and the engine of the whole
//! training plan.
//!
//! Walks each open signal forward through the small candles until price hits
//! the stop or the target, then records the result, how big it was in R, and
//! how far price ran each way before resolving.
//!
//! When both got hit inside one candle, the result is `ambiguous` and gets
//! left out of the numbers rather than resolved in your favour.
//!
//! This job is why the system gets better instead of merely running. No
//! tracker means no dataset, no dataset means no model, and Phase 4 never
//! happens.
//!
//! It is also the least interesting job to build — which is exactly why it
//! tends to get put off until the data it should have been collecting is gone.
