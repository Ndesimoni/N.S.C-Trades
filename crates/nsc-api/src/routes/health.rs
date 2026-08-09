//! Is it alive, and is it working?
//!
//! Reports more than "the program is running": whether the database and Redis
//! are reachable, and the last candle received for each pair.
//!
//! A bot that is running but no longer receiving prices is not working,
//! however cheerfully it answers a ping.
