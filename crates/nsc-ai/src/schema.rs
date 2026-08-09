//! The fixed shapes for questions and answers.
//!
//! Answers must come back in a set shape and get checked on arrival. A
//! confidence outside the valid range, or a response that will not parse,
//! counts as a failure and gets skipped — exactly like a timeout.
//!
//! The alternative — accepting free text and pulling it apart with string
//! matching somewhere downstream — is how a checking layer becomes the least
//! reliable part of a system built on fixed rules.
