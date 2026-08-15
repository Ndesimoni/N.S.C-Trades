//! Shared by the level tests.

use rust_decimal::Decimal;

pub(super) fn d(text: &str) -> Decimal {
    text.parse().expect("a number")
}

/// A scratch folder of its own, so tests cannot tread on each other.
pub(super) fn scratch(name: &str) -> std::path::PathBuf {
    let folder = std::env::temp_dir().join(format!("nsc-levels-{name}"));
    let _ = std::fs::remove_dir_all(&folder);
    folder
}
