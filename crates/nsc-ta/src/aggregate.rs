//! Building 1-hour, 4-hour and daily candles out of 15-minute ones.
//!
//! Why build them instead of asking the broker for them: control over when
//! the day ends. The daily close time — 5pm New York by convention, set in
//! `config/app.toml` — decides where every daily level sits. Brokers disagree
//! with each other about this, and a level that does not match the one on your
//! own chart destroys your trust in the bot faster than a losing trade does.
//!
//! A part-formed bigger candle must never be handed out. A 4-hour candle made
//! from only three 15-minute candles is not a 4-hour candle, and signalling on
//! it is using data you do not have yet.
