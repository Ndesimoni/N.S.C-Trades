//! Reading the whole file.

use std::path::Path;

use nsc_core::candle::Candle;
use nsc_core::error::CoreError;

use super::columns::Columns;
use super::rows::to_candle;
use crate::error::DataError;

/// Reads every candle in a CSV file, oldest first.
///
/// ## It stops on the first bad row rather than skipping it
///
/// A live feed sending one broken candle should be shrugged off — the feed
/// will send another. A file is the same every time it is read, so a bad row
/// is a broken file. Skipping it quietly would change every level built from
/// that file, with nothing anywhere to show that it happened.
///
/// ## Order is checked, not assumed
///
/// Candles out of order silently break every swing, level and trendline built
/// from them, and nothing else in the system would ever report it. Some
/// exports come newest-first; those are refused rather than reversed, because
/// a file that is *nearly* sorted would be silently accepted by anything
/// cleverer.
pub fn read_candles(path: &Path) -> Result<Vec<Candle>, DataError> {
    let mut file = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(path)
        .map_err(|why| DataError::CannotRead {
            path: path.to_path_buf(),
            detail: why.to_string(),
        })?;

    let header: Vec<String> = file
        .headers()
        .map_err(|why| DataError::BadHeader {
            path: path.to_path_buf(),
            detail: why.to_string(),
        })?
        .iter()
        .map(str::to_string)
        .collect();

    let at = Columns::from_header(&header, path)?;
    let mut candles: Vec<Candle> = Vec::new();

    for (index, row) in file.records().enumerate() {
        // Line 1 is the header, so the first row of data is line 2.
        let line = index + 2;

        let row = row.map_err(|why| DataError::BadRow {
            path: path.to_path_buf(),
            line,
            detail: why.to_string(),
        })?;

        let row: Vec<String> = row.iter().map(str::to_string).collect();
        let candle = to_candle(&row, at, line, path)?;

        if let Some(last) = candles.last()
            && candle.open_time() <= last.open_time()
        {
            return Err(DataError::Core(CoreError::CandlesOutOfOrder {
                arriving: candle.open_time(),
                last: last.open_time(),
            }));
        }

        candles.push(candle);
    }

    Ok(candles)
}
