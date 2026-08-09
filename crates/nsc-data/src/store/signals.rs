//! Saving every setup — the ones sent and the ones blocked.
//!
//! Blocked setups get saved on purpose. They answer "why was there nothing on
//! EURUSD this week?" without rerunning anything, and they are the "don't take
//! this one" examples the Phase 4 model trains against. A dataset of only sent
//! signals teaches a model nothing about what to refuse.
//!
//! The `features` column holds everything the bot saw at that moment, exactly
//! as it saw it. Never recalculate it later against updated chart-reading
//! code — the model would end up trained on inputs the live bot never
//! actually produced.
