//! The trained model that scores setups. Phase 4.
//!
//! Loads the model trained in `research/` and scores setups using the same
//! information that gets saved with every signal.
//!
//! It is a tree-based model, not a neural network. At this amount of data
//! trees win, and — more usefully — you can ask them which of your confluences
//! actually carried weight.
//!
//! ## What it learns
//!
//! Not your strategy. Your rules already are your strategy. It learns **which
//! of your setups to take and which to skip** — the filter you currently apply
//! by instinct.
//!
//! ## What it needs first
//!
//! Around 200–300 signals you have judged. Below that it learns noise, and it
//! will report high confidence while doing so. There is no shortcut: Phase 3 —
//! running the bot and pressing 👍/👎 — is what produces the data.
//!
//! The information fed in here must match `nsc-ta::snapshot` exactly. The
//! usual way this kind of component fails is a quiet mismatch between what it
//! was trained on and what it is given live. Nothing detects it, because both
//! sides keep working and only the scores are wrong.
