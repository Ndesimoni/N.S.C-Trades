//! Commands you can type in the chat.
//!
//!   /stats [period]   win rate, average result, count — live signals only
//!   /open             signals still waiting for a result
//!   /pause, /resume   stop and start sending
//!   /why <id>         the full reasoning behind one signal
//!   /health           is the feed alive? last candle for each pair
//!
//! `/health` matters more than it sounds. The usual failure of a system like
//! this is not a crash — it is a feed that quietly stopped while the program
//! kept running. Silence from the bot looks exactly like a quiet market, and
//! you can lose a week before you notice.
