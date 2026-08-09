//! Redis — the state that has to survive a restart.
//!
//! Redis is here for one specific job, not as a general cache. Postgres serves
//! candles perfectly well at this size. What lives in Redis is state you would
//! hate to lose when you redeploy:
//!
//!   - "already sent a EURUSD buy at this level"
//!   - per-pair cooldowns, so the same idea does not fire every candle
//!   - losing-streak and daily-loss state for the brakes
//!   - rate limit counters
//!   - messages between the background jobs
//!
//! Everything gets an expiry. A cooldown that outlives its purpose because
//! nothing cleaned it up leaves you with a bot that has gone silent for no
//! visible reason, which is genuinely unpleasant to work out.
