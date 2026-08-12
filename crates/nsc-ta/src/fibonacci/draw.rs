//! Picking which move to measure.

use chrono::{DateTime, Utc};
use nsc_core::fib::FibRetracement;
use nsc_core::swing::Swing;

use crate::error::TaError;

/// The move price is retracing right now: the last completed leg.
///
/// Takes the swings as [`crate::swings::find_swings`] gives them back — in
/// confirmation order — and measures between the last two.
///
/// ## Why the last two and nothing cleverer
///
/// Swings alternate, so the last two are always a low then a high or a high
/// then a low. That pair IS the move just completed, and it is the same run
/// the swing finder measured to confirm them.
///
/// Anything cleverer — the biggest recent move, the move in the direction of
/// the trend — would let two parts of the chart-reading code disagree about
/// what the current move is. That kind of disagreement stays invisible until
/// a signal looks wrong and nobody can say why.
///
/// ## Nothing before it could be known
///
/// Swings that had not confirmed by `now` are ignored, so a move can never be
/// drawn from a swing the market had not printed yet.
///
/// Gives back `None` when there are not two usable swings, which is the normal
/// state at the start of a history.
pub fn last_move(swings: &[Swing], now: DateTime<Utc>) -> Result<Option<FibRetracement>, TaError> {
    let mut known = swings.iter().filter(|swing| swing.is_known_at(now));

    let (Some(to), Some(from)) = (known.next_back(), known.next_back()) else {
        return Ok(None);
    };

    // A pair that is not one high and one low is not a leg. It cannot happen
    // with the current finder, which alternates — but it would be a silent
    // nonsense if it ever did, so it is refused rather than measured.
    if from.kind() == to.kind() {
        return Ok(None);
    }

    Ok(Some(FibRetracement::between(*from, *to)?))
}
