//! Finding swings one candle at a time.

use nsc_core::candle::Candle;
use nsc_core::swing::Swing;

use super::leg::Leg;
use super::memory::RunMemory;
use super::seed::Seed;
use super::step::Step;
use crate::config::SwingSettings;
use crate::error::TaError;

/// What the finder is doing right now.
#[derive(Debug, Clone)]
enum State {
    /// No swing has been confirmed yet, so the direction is worked out fresh
    /// from where price has been.
    Seeking(Seed),

    /// Anchored on a confirmed swing, following the run away from it.
    Running(Leg),
}

/// Finds swings as candles arrive.
///
/// Feed it complete candles in time order. It gives back any swings it can now
/// be sure about — usually none.
///
/// ## What proves a swing
///
/// A peak is not a peak because of how many candles sit either side of it. It
/// is a peak because price left it. So the finder measures the **run** from
/// the last confirmed swing, and then how much of that run gets given back:
///
///   - given back half → the peak is a swing, on its own
///   - given back the shallower share, and then price takes the peak out →
///     the peak is a swing, and so is the bottom of that pause
///
/// The second route is there because the strongest trends barely pause. A rule
/// that only confirmed on depth would read structure fine in a choppy market
/// and go blind in a clean trend.
///
/// ## Two things that follow from it
///
/// **Swings alternate.** After a high it is hunting a low. The same candle can
/// never be both.
///
/// **The wait is honest rather than fixed.** A swing is knowable when the
/// pullback gets there — sometimes two candles later, sometimes thirty. On the
/// daily a shallow pullback can leave a swing you can plainly see unusable for
/// weeks. That is the rule being strict, not a bug.
#[derive(Debug, Clone)]
pub struct SwingFinder {
    settings: SwingSettings,
    memory: RunMemory,
    state: Option<State>,
}

impl SwingFinder {
    pub fn new(settings: SwingSettings) -> Result<Self, TaError> {
        settings.validate()?;

        let memory = RunMemory::new(settings.run_memory_legs, settings.min_run_fraction);

        Ok(Self {
            settings,
            memory,
            state: None,
        })
    }

    /// Takes the next candle and gives back any swings now confirmed.
    ///
    /// Usually empty. Two come back together when a shallow pause is proved by
    /// price taking the peak out — the end of the run and the end of the
    /// pause are both known at that moment.
    pub fn update(&mut self, candle: &Candle) -> Result<Vec<Swing>, TaError> {
        if !candle.is_complete() {
            return Err(TaError::IncompleteCandle {
                open_time: candle.open_time(),
            });
        }

        let Some(state) = self.state.take() else {
            self.state = Some(State::Seeking(Seed::new(candle)));
            return Ok(Vec::new());
        };

        match state {
            State::Seeking(seed) => self.seek(seed, candle),
            State::Running(leg) => self.follow(leg, candle),
        }
    }

    /// Before the first swing, working out the direction as it goes.
    fn seek(&mut self, mut seed: Seed, candle: &Candle) -> Result<Vec<Swing>, TaError> {
        let step = seed.take(candle, &self.settings, &self.memory)?;

        if matches!(step, Step::Continue) {
            self.state = Some(State::Seeking(seed));
            return Ok(Vec::new());
        }

        self.settle(step)
    }

    /// After the first swing: one leg, anchored on it.
    fn follow(&mut self, mut leg: Leg, candle: &Candle) -> Result<Vec<Swing>, TaError> {
        let step = leg.take(candle, &self.settings, &self.memory)?;

        if matches!(step, Step::Continue) {
            self.state = Some(State::Running(leg));
            return Ok(Vec::new());
        }

        self.settle(step)
    }

    /// Records a finished run and starts following the next one.
    fn settle(&mut self, step: Step) -> Result<Vec<Swing>, TaError> {
        let Step::Confirmed { swings, next, run } = step else {
            return Ok(Vec::new());
        };

        self.memory.remember(run);
        self.state = Some(State::Running(next));

        Ok(swings)
    }
}
