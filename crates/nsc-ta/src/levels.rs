//! Support and resistance, built by grouping nearby swing points.
//!
//! Swing highs and lows that sit close together get merged into one zone. How
//! close counts as "close together" is measured against normal candle size,
//! not in pips, so the same setting behaves sensibly on EURUSD and GBPJPY.
//!
//! Each zone tracks how many times it has been touched, how old it is, which
//! timeframe it came from, and a strength score.
//!
//! Two behaviours worth knowing:
//!
//!   - **Levels fade.** A zone from 500 candles ago that price never revisited
//!     is not the level it used to be.
//!   - **Levels wear out.** A zone tested four or five times is *more* likely
//!     to break than to hold, so `exhausted` is exposed and the rules can
//!     refuse it. Most simple support/resistance code gets this backwards and
//!     treats lots of touches as strength.
//!
//! Levels from higher timeframes get carried down too, since an hourly setup
//! sitting on a 4-hour or daily level is a different trade.
