//! Pulling the numbers out of a message.

use rust_decimal::Decimal;

/// Every number in the message.
///
/// One per line, several on a line, or one on its own — whatever is there.
///
/// **Nothing asks how many.** A count is one more thing to get wrong: say four
/// and send three and the bot waits forever; say four and send five and one
/// gets dropped.
pub fn prices_in(text: &str) -> Vec<Decimal> {
    text.split_whitespace()
        .filter_map(|word| word.replace(',', "").parse::<Decimal>().ok())
        .collect()
}
