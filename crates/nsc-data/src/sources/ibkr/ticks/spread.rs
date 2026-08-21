//! The last bid, the last ask, and the middle of the two.

use ibapi::contracts::tick_types::TickType;
use rust_decimal::Decimal;

use super::super::candles::price;

/// What is currently on offer for one pair.
///
/// **IBKR never sends a price. It sends a bid, and separately an ask.** Twelve
/// Data sent one number a second and this project never had to think about it:
///
/// ```text
///     Twelve Data     {"price": "1.16413"}          one message, one number
///     IBKR            Bid  1.16412                  two messages, and they
///                     Ask  1.16414                  arrive at different times
/// ```
///
/// So the middle has to be worked out here, and it cannot be worked out at all
/// until both sides have arrived once.
#[derive(Debug, Default)]
pub(super) struct Spread {
    bid: Option<Decimal>,
    ask: Option<Decimal>,
    last_said: Option<Decimal>,
}

impl Spread {
    /// Take one tick, and give back the new middle **only if it moved**.
    ///
    /// **Two ticks make one price.** A bid and an ask arriving a moment apart
    /// describe the same market, so reporting both would double the traffic on
    /// the loop for no new information.
    ///
    /// Nothing comes back until both sides have been seen. A middle worked out
    /// from a bid alone is not the middle of anything.
    pub(super) fn took(&mut self, which: TickType, raw: f64) -> Option<Decimal> {
        // **IBKR sends -1 to mean "no price", not "the price is minus one".**
        // Let one through and a level would look like it had been reached from
        // an impossible distance below.
        //
        // `is_finite` is checked as well as the sign, because a NaN compares
        // false against everything — so `raw <= 0.0` on its own would wave one
        // straight through, and the middle of anything and a NaN is a NaN.
        if !raw.is_finite() || raw <= 0.0 {
            return None;
        }

        let value = price("tick", raw).ok()?;

        match which {
            TickType::Bid => self.bid = Some(value),
            TickType::Ask => self.ask = Some(value),

            // **Delayed prices are refused, on purpose.** They are fifteen
            // minutes behind. An alert saying price is at his level when it was
            // there a quarter of an hour ago is worse than no alert, because he
            // would act on it. `listening.rs` says so out loud rather than
            // going quiet.
            _ => return None,
        }

        let middle = (self.bid? + self.ask?) / Decimal::TWO;

        // Barely moving is the normal state — prices arrive constantly and
        // almost all of them describe the same market as the last one.
        if self.last_said == Some(middle) {
            return None;
        }

        self.last_said = Some(middle);

        Some(middle)
    }
}
