//! Things that go wrong in the AI layer.
//!
//! Every one of these is designed to **fail safe**. Timeout, rate limit,
//! garbled answer, wrong shape — all mean "check skipped", noted on the
//! signal, and the setup goes out on the strength of the chart.
//!
//! That is deliberate and worth defending. This layer is advice. A good setup
//! should not be lost to someone else's outage, and a component that can
//! quietly become the single thing breaking your whole signal flow is worse
//! than not having it.
//!
//! The exception is the Phase 4 model. If it fails to **load** at startup,
//! that is fatal — running with scoring switched on but no model quietly gives
//! you a different system from the one you configured.
