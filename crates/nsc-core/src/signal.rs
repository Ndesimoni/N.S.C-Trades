//! Setups, signals, and what happens to them.
//!
//! `Candidate` is what the rules engine produces: direction, entry, stop,
//! target, risk-to-reward, a confidence score, and `reasons` — the plain list
//! that becomes the "why" in your Telegram message. A setup that cannot
//! explain itself in one sentence means the rules are too loose.
//!
//! `Signal` is a setup that passed the checks and got recorded. Its status
//! says why it was or was not sent — score too low, news, risk limit,
//! cooldown, AI concern.
//!
//! Setups that got blocked are saved too, not thrown away. They are the
//! "don't take this one" examples the Phase 4 model needs.
//!
//! `features` holds everything the bot saw at that moment. It is saved as-is
//! rather than worked out again later — recalculating it against updated
//! chart-reading code would train the model on inputs the live bot never
//! actually produced.
