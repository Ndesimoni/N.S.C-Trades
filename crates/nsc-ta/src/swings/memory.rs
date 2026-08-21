//! The recent runs, and whether the next one is big enough to count.

use std::collections::VecDeque;

use rust_decimal::Decimal;

use super::Rules;

/// The last few runs the market actually made.
///
/// **A move is only a move next to the moves around it.** A 40-point wobble
/// after a 200-point leg is not structure — and it does not matter which
/// instrument or timeframe that happens on, which is why the floor is a share
/// of recent runs rather than a number of pips or of normal candles.
#[derive(Debug, Clone)]
pub(super) struct RunMemory {
    recent: VecDeque<Decimal>,
    keep: usize,
    least: Decimal,
}

impl RunMemory {
    pub(super) fn new(rules: &Rules) -> Self {
        RunMemory {
            recent: VecDeque::with_capacity(rules.run_memory_legs),
            keep: rules.run_memory_legs.max(1),
            least: rules.min_run_fraction,
        }
    }

    /// Is this run big enough to be treated as a move?
    ///
    /// **The first run always passes.** With nothing to compare against,
    /// refusing it would mean the bot could never start.
    ///
    /// **Compared against the BIGGEST run remembered, not the last one.** Each
    /// run being half the one before it would otherwise pass forever while the
    /// chain shrank to nothing — 200, 120, 72, 43, 26. Against the biggest,
    /// the third one already fails and the shrinking stops.
    pub(super) fn allows(&self, run: Decimal) -> bool {
        let Some(biggest) = self.recent.iter().copied().max() else {
            return true;
        };

        run >= biggest * self.least
    }

    /// Remember a run that counted.
    pub(super) fn saw(&mut self, run: Decimal) {
        if self.recent.len() == self.keep {
            self.recent.pop_front();
        }

        self.recent.push_back(run);
    }
}
