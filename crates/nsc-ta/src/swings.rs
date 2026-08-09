//! Finding swing highs and lows — the foundation of everything.
//!
//! Spend more time here than anywhere else. Levels, trendlines, Fibonacci
//! anchors, trend direction and chart patterns are all built on this one
//! output. Get it right and most of the engine follows. Get the sensitivity
//! wrong and every feature downstream is quietly rubbish, in a way that is
//! very hard to trace back.
//!
//! ## How it works
//!
//! A swing high is a candle whose high beats the highs of a few candles on
//! either side. Same idea upside-down for lows. Small moves get filtered out
//! by requiring a minimum size relative to a normal candle, so that choppy
//! noise does not register as structure.
//!
//! ## Confirmation — read this before changing anything
//!
//! A swing at candle 100 is not knowable until candle 103 has printed. So
//! this module tags every swing with `confirmed_at`, and callers must respect
//! it.
//!
//! Feeding an unconfirmed swing into level detection is the easiest possible
//! way to produce a beautiful backtest you cannot trade.
//!
//! ## Tuning
//!
//! The lookback setting is the single most influential number in the project.
//! Test it properly in the backtester. Never nudge it because one chart looks
//! nicer.
