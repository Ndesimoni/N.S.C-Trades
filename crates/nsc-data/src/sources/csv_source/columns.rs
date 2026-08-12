//! Working out which column is which.

use std::path::Path;

use crate::error::DataError;

/// Where each price lives in a row.
#[derive(Debug, Clone, Copy)]
pub(super) struct Columns {
    pub time: usize,
    pub open: usize,
    pub high: usize,
    pub low: usize,
    pub close: usize,
}

impl Columns {
    /// Reads the header and finds the five columns that matter.
    ///
    /// Names are matched case-insensitively, and the usual spellings from
    /// TradingView, MetaTrader and broker exports are all understood.
    ///
    /// A missing column is refused rather than guessed at. Guessing would give
    /// a file that parses perfectly and is wrong in every price, which is the
    /// worst possible outcome — nothing downstream could notice.
    pub fn from_header(header: &[String], path: &Path) -> Result<Self, DataError> {
        let named = |wanted: &[&str]| -> Option<usize> {
            header.iter().position(|column| {
                let column = column.trim().to_ascii_lowercase();
                let column = column.trim_start_matches('\u{feff}');
                wanted.contains(&column)
            })
        };

        let missing = |what: &str| DataError::BadHeader {
            path: path.to_path_buf(),
            detail: format!("no {what} column — found: {}", header.join(", ")),
        };

        Ok(Self {
            time: named(&[
                "time",
                "timestamp",
                "date",
                "datetime",
                "open_time",
                "local time",
            ])
            .ok_or_else(|| missing("time"))?,
            open: named(&["open", "o"]).ok_or_else(|| missing("open"))?,
            high: named(&["high", "h"]).ok_or_else(|| missing("high"))?,
            low: named(&["low", "l"]).ok_or_else(|| missing("low"))?,
            close: named(&["close", "c", "close/last"]).ok_or_else(|| missing("close"))?,
        })
    }
}
