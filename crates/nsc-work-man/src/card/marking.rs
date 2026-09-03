//! **What tells three cards apart from any other card.**
//!
//! A setup arrives as three separate messages, and they land in a chat that
//! also carries close cards, news cards and charts he asked for. Nothing said
//! which three belonged together.
//!
//! His idea, 3 September 2026: *"put them inside a dotted box frame."*
//!
//! ## And a counter with it, because the frame alone is not enough
//!
//! A dashed frame on one card looks exactly like a dashed frame on one of
//! three. It says *this is a setup*; it does not say *and there are two more
//! below*. So each card also carries **1/3**, and the last one is the only
//! one with the buttons.
//!
//! Nothing else the bot sends is framed, so the frame means one thing.

use serde_json::{Value, json};

/// Which card of a setup this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part {
    pub which: u8,
    pub of: u8,
}

impl Part {
    /// The first of three, the second, the third.
    pub fn of_three(which: u8) -> Self {
        Part { which, of: 3 }
    }
}

/// What a chart is marked with, beyond the candles themselves.
///
/// **One struct rather than two more arguments.** The renderer was already at
/// the point where clippy had to be silenced about its argument count, which
/// is the compiler saying what a reader would.
#[derive(Debug, Clone, Copy, Default)]
pub struct Mark {
    /// Ring the last `n` candles in red. `None` on the wide chart — a ring at
    /// the far right of two hundred candles points at nothing readable.
    pub ring: Option<usize>,

    /// **`None` means this is not part of a setup**, and it gets no frame.
    /// A chart he asked for is not a signal and must not wear the badge of
    /// one.
    pub part: Option<Part>,
}

impl Mark {
    /// A chart that belongs to nothing — a review chart, or a preview with no
    /// shape on it.
    pub fn plain() -> Self {
        Mark::default()
    }

    /// Part of a setup, with no ring.
    pub fn part(which: u8) -> Self {
        Mark {
            ring: None,
            part: Some(Part::of_three(which)),
        }
    }

    /// Part of a setup, with the shape ringed.
    pub fn ringed(which: u8, candles: usize) -> Self {
        Mark {
            ring: Some(candles),
            part: Some(Part::of_three(which)),
        }
    }

    /// What the template is handed. `null` when it belongs to nothing.
    pub(super) fn as_json(self) -> Value {
        match self.part {
            None => Value::Null,
            Some(part) => json!({ "which": part.which, "of": part.of }),
        }
    }
}
