//! High or low.

/// Which kind of swing this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwingKind {
    /// A peak. Price came up to it and left.
    High,

    /// A trough.
    Low,
}

impl SwingKind {
    /// The other one.
    ///
    /// **After a high the finder is looking for a low.** That is what makes
    /// swings alternate, and it is why this is a method rather than a `match`
    /// written out at each call site.
    pub fn opposite(self) -> Self {
        match self {
            Self::High => Self::Low,
            Self::Low => Self::High,
        }
    }

    /// What to call it to a person.
    pub fn spoken(self) -> &'static str {
        match self {
            Self::High => "swing high",
            Self::Low => "swing low",
        }
    }
}
