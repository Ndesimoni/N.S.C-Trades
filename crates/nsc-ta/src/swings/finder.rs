//! Finding swings one candle at a time.

use nsc_core::candle::{Bar, CandleError};
use nsc_core::swing::{Swing, SwingError};
use thiserror::Error;

use super::Rules;
use super::leg::Leg;
use super::memory::RunMemory;
use super::seed::Seed;
use super::step::Step;

/// What the finder is doing right now.
#[derive(Debug, Clone)]
enum State {
    /// No swing has been confirmed yet, so the direction is worked out fresh
    /// from where price has been.
    Seeking(Seed),

    /// Anchored on a confirmed swing, following the run away from it.
    Running(Leg),
}

/// What can go wrong finding swings.
#[derive(Debug, Error)]
pub enum SwingsError {
    #[error(transparent)]
    NotACandle(#[from] CandleError),

    #[error(transparent)]
    NotASwing(#[from] SwingError),
}

/// Finds swings as candles arrive.
///
/// **Feed it finished candles in time order.** It gives back any swings it can
/// now be sure about — usually none.
///
/// ## What proves a swing
///
/// A peak is not a peak because of how many candles sit either side of it. It
/// is a peak **because price left it**. So the finder measures the run from
/// the last confirmed swing, and then how much of that run gets given back:
///
/// ```text
///     given back half                       the peak is a swing, on its own
///     given back the shallower share,
///       and then price takes the peak out   the peak is a swing, and so is
///                                           the bottom of that pause
/// ```
///
/// **The second route is there because the strongest trends barely pause.** A
/// rule that only confirmed on depth would read structure fine in a choppy
/// market and go blind in a clean trend.
///
/// ## It cannot see forwards
///
/// Candles go in one at a time and swings come out when they prove themselves.
/// There is no argument that could hand it a candle from later, so the rule
/// that matters most here is the shape of the thing rather than a discipline.
#[derive(Debug, Clone)]
pub struct Finder {
    rules: Rules,
    memory: RunMemory,
    state: Option<State>,
}

impl Finder {
    pub fn new(rules: Rules) -> Self {
        Finder {
            memory: RunMemory::new(&rules),
            rules,
            state: None,
        }
    }

    /// Take one finished candle. **Usually gives back nothing.**
    pub fn take(&mut self, bar: &Bar) -> Result<Vec<Swing>, SwingsError> {
        let now = bar.opened_at()?;

        let Some(state) = self.state.take() else {
            self.state = Some(State::Seeking(Seed::new(bar, now)));
            return Ok(Vec::new());
        };

        let (step, state) = match state {
            State::Seeking(mut seed) => {
                let step = seed.take(bar, now, &self.rules, &self.memory)?;
                (step, State::Seeking(seed))
            }
            State::Running(mut leg) => {
                let step = leg.take(bar, now, &self.rules, &self.memory)?;
                (step, State::Running(leg))
            }
        };

        match step {
            Step::Continue => {
                self.state = Some(state);
                Ok(Vec::new())
            }

            Step::Confirmed { swings, next, run } => {
                self.memory.saw(run);
                self.state = Some(State::Running(next));
                Ok(swings)
            }
        }
    }

    /// Every swing in a run of candles, oldest first.
    ///
    /// **A convenience, and it changes nothing.** It feeds them in one at a
    /// time exactly as the live bot would, so a swing here is confirmed on the
    /// same candle it would be confirmed on live.
    pub fn over(rules: Rules, bars: &[Bar]) -> Result<Vec<Swing>, SwingsError> {
        let mut finder = Finder::new(rules);
        let mut found = Vec::new();

        for bar in bars {
            found.extend(finder.take(bar)?);
        }

        Ok(found)
    }
}
