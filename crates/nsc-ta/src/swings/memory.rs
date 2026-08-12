//! The recent runs, and whether the next one is big enough to count.

use std::collections::VecDeque;

use nsc_core::price::PriceDistance;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

/// The last few runs the market actually made.
///
/// A move is only a move next to the moves around it. A 40-point wobble after
/// a 200-point leg is not structure, and it does not matter which instrument
/// or timeframe that happens on — which is why the floor is a share of recent
/// runs rather than a number of pips or of normal candles.
#[derive(Debug, Clone)]
pub(super) struct RunMemory {
    recent: VecDeque<PriceDistance>,
    keep: usize,
    min_fraction: f64,
}

impl RunMemory {
    pub fn new(keep: usize, min_fraction: f64) -> Self {
        Self {
            recent: VecDeque::with_capacity(keep),
            keep,
            min_fraction,
        }
    }

    /// Is this run big enough to be treated as a move?
    ///
    /// The first run always passes — with nothing to compare against, refusing
    /// it would mean the bot could never start.
    ///
    /// Compared against the **biggest** run remembered, not the last one. Each
    /// run being half the one before it would otherwise pass forever while the
    /// chain shrank to nothing: 200, 120, 72, 43, 26. Against the biggest, the
    /// third one already fails and the shrinking stops.
    pub fn allows(&self, run: PriceDistance) -> bool {
        let Some(biggest) = self.biggest() else {
            return true;
        };

        // Only fails if the fraction is not a real number, which validate()
        // has already refused. Let the run through rather than silently
        // swallowing every swing.
        let Some(fraction) = Decimal::from_f64(self.min_fraction) else {
            return true;
        };

        run.value() >= biggest.value() * fraction
    }

    /// Remembers a run that counted. Rejected runs never go in — otherwise a
    /// quiet stretch would slowly redefine what a big move is.
    pub fn remember(&mut self, run: PriceDistance) {
        if self.recent.len() == self.keep {
            self.recent.pop_front();
        }

        self.recent.push_back(run);
    }

    fn biggest(&self) -> Option<PriceDistance> {
        self.recent
            .iter()
            .copied()
            .max()
            .filter(|biggest| biggest.value() > Decimal::ZERO)
    }
}
