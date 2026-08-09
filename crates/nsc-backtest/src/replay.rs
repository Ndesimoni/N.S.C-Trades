//! Feeding old candles through the same pipeline the live bot uses.
//!
//! Sends candles in strict time order across all pairs at once, so that
//! portfolio-wide rules — correlation limits, exposure caps, losing-streak
//! brakes — see the same mix they would see live.
//!
//! Replaying one pair at a time is faster and quietly wrong: it makes every
//! correlation limit impossible to enforce, so the backtest measures a
//! portfolio you cannot actually run.
//!
//! Bigger timeframes get built as the replay goes rather than read in advance,
//! so a 4-hour candle only becomes visible when its last 15-minute candle
//! closes — exactly as it does live.
