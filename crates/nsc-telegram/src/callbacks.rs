//! Handling button presses — this is what writes your verdicts down.
//!
//! Checks the press is genuine, finds the signal, saves your verdict, and
//! confirms so the button stops spinning.
//!
//! Saving **replaces** rather than adds. Changing your mind is normal and the
//! latest verdict wins. A table full of contradictory verdicts for the same
//! signal would train a model on your indecision.
//!
//! Press the button when the signal arrives, or not at all. A verdict pressed
//! a week later is hindsight, not judgement — by then you know whether it won,
//! and a model trained on that has learned to predict the past.
