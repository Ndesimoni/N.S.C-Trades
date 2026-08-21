//! Which pairs IBKR has refused, and the point at which that is fatal.

use std::collections::BTreeMap;

/// The pairs IBKR has said no to since the line opened.
///
/// **A refusal does not arrive as an error.** IBKR sends a notice down a line
/// that stays open and then never sends a price for that pair — so one refused
/// pair is silent in exactly the way a quiet pair is silent. This is what
/// notices its own silence.
pub(super) struct Refusals {
    asked: usize,

    /// Kept by symbol, so the same pair complaining twice is still one pair.
    /// Sorted, so the message does not shuffle between readings.
    said: BTreeMap<String, String>,
}

impl Refusals {
    pub(super) fn watching(asked: usize) -> Self {
        Refusals {
            asked,
            said: BTreeMap::new(),
        }
    }

    /// Remembers one refusal, and answers: **is that all of them?**
    ///
    /// True means nothing will ever arrive on this line and it is not worth
    /// holding open.
    ///
    /// **Nought refused out of nought asked is not a total refusal.** Removing
    /// his last pair left the watcher asking about nothing, `0 == 0` came out
    /// true, and it reported every pair as refused — which read as the price
    /// line breaking, every thirty seconds, over a bot doing exactly what it
    /// had been told.
    pub(super) fn and_that_is_everything(&mut self, symbol: &str, why: &str) -> bool {
        let first_time = self
            .said
            .insert(symbol.to_string(), why.to_string())
            .is_none();

        // Not fatal on its own — the other pairs are still being watched. But
        // it must be said, because one refused pair looks exactly like one
        // quiet pair.
        if first_time {
            eprintln!("IBKR will not send prices for {symbol}: {why}");
        }

        self.asked > 0 && self.said.len() >= self.asked
    }

    /// Every refusal, for the one message that says the line is dead.
    pub(super) fn what_they_said(&self) -> String {
        self.said
            .iter()
            .map(|(symbol, why)| format!("{symbol} ({why})"))
            .collect::<Vec<_>>()
            .join("; ")
    }
}
