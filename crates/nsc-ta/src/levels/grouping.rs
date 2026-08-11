//! Sliding one band to catch the most swing points.

use nsc_core::price::PriceDistance;
use nsc_core::swing::Swing;

/// The swing points one band can cover, and how tightly they sit.
///
/// A group always has at least one swing in it. `lowest` is kept as its own
/// field so that stays true for the compiler as well as on paper — whoever
/// builds a level from this never has to handle an empty group that cannot
/// happen.
pub(super) struct Group {
    /// The cheapest swing in the group. Also the first one in `swings`.
    pub lowest: Swing,

    pub swings: Vec<Swing>,

    /// From the lowest swing in the group to the highest. Never more than the
    /// band's thickness.
    pub spread: PriceDistance,
}

/// Finds the busiest place to put one band.
///
/// Returns the group of swing points a single band of `thickness` can cover,
/// choosing the position that catches the most. `None` when there are no
/// swings left to group.
///
/// ## How it slides
///
/// The swings get sorted by price. Then, starting from each one in turn, it
/// takes everything within one thickness above it. That is every position the
/// band could usefully sit in — sliding it any further only drops the swing
/// at the bottom without picking up a new one at the top.
///
/// ## When two positions catch the same number
///
/// The tighter one wins — the group whose swings sit closest together. If
/// they are equally tight, the lower one wins.
///
/// That last rule is arbitrary, and it is here on purpose. Without it the
/// answer could depend on the order the swings arrived in, and then the same
/// history would produce different levels on different runs. A backtest you
/// cannot repeat tells you nothing.
pub(super) fn best_group(swings: &[Swing], thickness: PriceDistance) -> Option<Group> {
    let mut sorted: Vec<Swing> = swings.to_vec();
    sorted.sort_by_key(|swing| swing.price());

    let mut best: Option<Group> = None;

    for (start, lowest) in sorted.iter().enumerate() {
        let mut end = start;

        while let Some(next) = sorted.get(end + 1) {
            if next.price() - lowest.price() > thickness {
                break;
            }
            end += 1;
        }

        let caught = &sorted[start..=end];
        let spread = match caught.last() {
            Some(highest) => highest.price() - lowest.price(),
            None => continue,
        };

        if beats(caught.len(), spread, best.as_ref()) {
            best = Some(Group {
                lowest: *lowest,
                swings: caught.to_vec(),
                spread,
            });
        }
    }

    best
}

/// Is this position better than the best one so far?
///
/// More touches first. Then tighter. Anything else is not an improvement —
/// which is what makes the lowest of several equally good positions win, since
/// the search runs upwards.
fn beats(touches: usize, spread: PriceDistance, best: Option<&Group>) -> bool {
    match best {
        None => true,
        Some(best) => match touches.cmp(&best.swings.len()) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => spread < best.spread,
        },
    }
}
