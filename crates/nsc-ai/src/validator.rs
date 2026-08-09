//! Asking an AI to look over a setup. Advice, never the deciding vote.
//!
//! It receives facts the bot already worked out — trend, what kind of level,
//! how old it is, how far away price is, the pattern, the risk-to-reward, the
//! session, upcoming news, recent headlines — and returns a confidence, a
//! short explanation, and any concerns.
//!
//! How the answer gets used matters as much as the answer. It can **lower**
//! confidence and add a warning to your message. It cannot rescue a setup that
//! failed one of your pass-or-fail rules.
//!
//! Letting a model talk the system into a trade your rules rejected would
//! quietly make the model your strategy — which is exactly what this whole
//! design exists to prevent.
//!
//! Every answer is saved, so you can check later whether its opinions were any
//! good. An adviser nobody checks accumulates trust it has not earned.
