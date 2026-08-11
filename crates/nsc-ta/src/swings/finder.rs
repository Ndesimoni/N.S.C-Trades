//! Finding swings one candle at a time.

use std::collections::VecDeque;

use nsc_core::candle::Candle;
use nsc_core::price::{AtrMultiple, PriceDistance};
use nsc_core::swing::{Swing, SwingKind};

use crate::config::SwingSettings;
use crate::error::TaError;
use crate::indicators::atr::Atr;

/// A candle, and how big a normal candle was at the time.
type Seen = (Candle, Option<PriceDistance>);

/// Finds swings as candles arrive.
///
/// Feed it complete candles in time order. It gives back any swings it can
/// now be sure about — usually none.
///
/// ## Why it always answers about an older candle
///
/// To know candle 100 was a peak, you need to see the candles after it. So
/// this keeps a small window of recent candles and only ever decides about
/// the one in the middle.
///
/// With a lookback of 3 the window is 7 candles wide, and when candle 103
/// arrives the finder decides about candle 100.
///
/// That lag is not a limitation to work around. It is the honest answer.
/// Anything faster is reading the chart backwards.
#[derive(Debug, Clone)]
pub struct SwingFinder {
    settings: SwingSettings,
    atr: Atr,

    /// The last `2 x lookback + 1` candles, oldest first.
    window: VecDeque<Seen>,
}

impl SwingFinder {
    pub fn new(settings: SwingSettings, atr_period: usize) -> Result<Self, TaError> {
        settings.validate()?;

        let width = 2 * settings.lookback + 1;

        Ok(Self {
            settings,
            atr: Atr::new(atr_period)?,
            window: VecDeque::with_capacity(width),
        })
    }

    /// How many candles the window holds when it is full.
    fn width(&self) -> usize {
        2 * self.settings.lookback + 1
    }

    /// Takes the next candle and gives back any swings now confirmed.
    ///
    /// Usually empty. A candle can be both a swing high and a swing low at
    /// once — an outside bar that beats everything around it in both
    /// directions — which is why this returns a list rather than one swing.
    ///
    /// An empty `Vec` does not allocate, so the common case is free.
    pub fn update(&mut self, candle: &Candle) -> Result<Vec<Swing>, TaError> {
        if !candle.is_complete() {
            return Err(TaError::IncompleteCandle {
                open_time: candle.open_time(),
            });
        }

        let atr_now = self.atr.update(candle)?;
        self.window.push_back((*candle, atr_now));

        if self.window.len() > self.width() {
            self.window.pop_front();
        }

        if self.window.len() < self.width() {
            return Ok(Vec::new());
        }

        self.decide_about_the_middle()
    }

    /// Looks at the candle in the middle of the window and works out whether
    /// it is a swing.
    fn decide_about_the_middle(&self) -> Result<Vec<Swing>, TaError> {
        let middle_index = self.settings.lookback;

        let Some((middle, atr_then)) = self.window.get(middle_index).copied() else {
            return Ok(Vec::new());
        };

        // The newest candle in the window is the one that made this
        // knowable. Analysis runs when a candle closes, so by the time we
        // are looking at that candle it has finished.
        let Some((newest, _)) = self.window.back().copied() else {
            return Ok(Vec::new());
        };

        // No ATR yet means we are at the very start of the history and have
        // no idea what a normal candle looks like. Better to find no swings
        // than to guess.
        let Some(atr_then) = atr_then else {
            return Ok(Vec::new());
        };

        let mut found = Vec::new();

        let highest = self.beats_neighbours(middle_index, SwingKind::High);
        let lowest = self.beats_neighbours(middle_index, SwingKind::Low);

        if let Some(stands_out_by) = highest
            && self.is_big_enough(stands_out_by, atr_then)?
        {
            found.push(Swing::new(
                SwingKind::High,
                middle.open_time(),
                newest.open_time(),
                middle.high(),
            )?);
        }

        if let Some(stands_out_by) = lowest
            && self.is_big_enough(stands_out_by, atr_then)?
        {
            found.push(Swing::new(
                SwingKind::Low,
                middle.open_time(),
                newest.open_time(),
                middle.low(),
            )?);
        }

        Ok(found)
    }

    /// Does the middle candle beat every other candle in the window, and by
    /// how much?
    ///
    /// Returns `None` if it does not. Strictly beat — a tie does not count.
    /// See README.txt for what that misses and why it is the safer choice.
    fn beats_neighbours(&self, middle_index: usize, kind: SwingKind) -> Option<PriceDistance> {
        let (middle, _) = self.window.get(middle_index)?;

        let mut nearest_rival = None;

        for (position, (other, _)) in self.window.iter().enumerate() {
            if position == middle_index {
                continue;
            }

            let (mine, theirs) = match kind {
                SwingKind::High => (middle.high(), other.high()),
                SwingKind::Low => (other.low(), middle.low()),
            };

            if theirs >= mine {
                return None;
            }

            let gap = mine - theirs;
            nearest_rival = Some(match nearest_rival {
                None => gap,
                Some(smallest) if gap < smallest => gap,
                Some(smallest) => smallest,
            });
        }

        nearest_rival
    }

    /// Is the swing worth calling a swing, or is it just chop?
    ///
    /// Measured against ATR rather than a fixed number of pips, so the same
    /// setting works on EURUSD and gold.
    fn is_big_enough(
        &self,
        stands_out_by: PriceDistance,
        atr: PriceDistance,
    ) -> Result<bool, TaError> {
        let threshold = AtrMultiple::new(self.settings.min_atr_multiple).to_distance(atr)?;

        Ok(stands_out_by >= threshold)
    }
}
