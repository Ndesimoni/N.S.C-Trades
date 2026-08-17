//! What came back from a drawing, and how much of it he can actually see.

use std::path::PathBuf;

use nsc_core::levels::Band;
use rust_decimal::Decimal;

/// A drawn chart, and whether any of his levels are actually on it.
///
/// **The count is the point.** A 4-hour chart is about twenty-five days wide,
/// and a weekly level drawn two years ago sits far off the top or bottom of
/// it. The picture draws perfectly and comes out looking empty — so the caller
/// is told, and can say so instead of sending something that looks broken.
pub struct Drawn {
    pub picture: PathBuf,

    /// How many of his bands fall inside the range of candles drawn.
    pub on_it: usize,

    /// How many the pair has altogether.
    pub altogether: usize,
}

/// How many bands overlap the price the chart covers.
///
/// **Overlap, not the middle.** A band whose line sits above the highest
/// candle can still have its lower edge on screen, and that edge is the part
/// he is looking for. Asking whether the line itself is on the chart would
/// report "nothing here" over a band he can plainly see.
pub(super) fn on_the_chart(bands: &[Band], low: Decimal, high: Decimal) -> usize {
    bands
        .iter()
        .filter(|band| band.top >= low && band.bottom <= high)
        .count()
}
