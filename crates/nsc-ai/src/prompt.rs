//! Building the questions sent to the AI, and versioning them.
//!
//! Questions are built from structured facts, never from free-form text
//! assembled elsewhere, so you can always reconstruct exactly what was asked.
//!
//! Versioned, because changing the wording quietly changes behaviour and no
//! test catches it. The version gets saved with every answer. Without it,
//! comparing this month's AI-blocked signals against last month's is comparing
//! two different reviewers.
