//! Why the line stopped being held open.

/// Why the line stopped being held open.
///
/// **Neither of these is a fault.** Trouble leaves by the error path, so that
/// the five-minute rule in `trouble.rs` can decide whether he hears about it.
pub enum Closed {
    /// The session ended, or he removed the last pair. Nothing is wrong.
    Line,

    /// A levels file changed. He has sent something.
    LevelsChanged,
}
