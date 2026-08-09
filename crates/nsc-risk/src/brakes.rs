//! Stopping the bot when things are going badly.
//!
//! Trips after a run of losses, or when the day's or week's losses hit a
//! limit.
//!
//! Two design choices. A tripped brake **tells you** rather than going quietly
//! silent — a bot that stops without saying why looks identical to a bot that
//! crashed. And restarting is manual: automatic resumption removes the moment
//! of review that was the entire point of stopping.
//!
//! Brake state lives in Redis so it survives a redeploy. A brake that resets
//! itself because you restarted the program is not a brake.
