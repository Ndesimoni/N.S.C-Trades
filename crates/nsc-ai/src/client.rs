//! Talking to the Claude API — keys, retries, timeouts, cost tracking.
//!
//! The important part is what happens when it fails. If the API is slow or
//! down, the check is **skipped** and the signal goes out on the strength of
//! the chart alone, with the skip noted on the record.
//!
//! Failing this way is right because this layer is only advice. A perfectly
//! good setup should not be lost to someone else's outage — and a checking
//! layer that can quietly become the single thing breaking your whole signal
//! flow is worse than having no checking layer.
