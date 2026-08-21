//! Asking IBKR whether it has ever heard of an instrument.

/// What IBKR said about a symbol.
///
/// **Two answers here, and a third in the `Err`.** That third one is the whole
/// reason this is not a `bool`:
///
/// ```text
///     Ok(Yes)        it serves it
///     Ok(Never)      it ANSWERED, and has never heard of it
///     Err(..)        it could not be asked at all
/// ```
///
/// A `bool` would fold the last two together, and they must never be folded.
/// "Never heard of it" is grounds for refusing a pair and for retiring a file
/// he already has. "Could not ask" happens every time TWS is shut — and if
/// that ever read as "never heard of it", one gateway outage would sweep away
/// every pair he owns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Serves {
    /// IBKR knows this instrument and will quote it.
    Yes,

    /// IBKR answered and does not know it. **This one is definite.**
    Never { why: String },
}

/// The TWS codes that mean "there is no such instrument".
///
/// **Only 200 is on this list, and it is meant to stay short.** Every code NOT
/// here is treated as "could not ask", which is the safe direction: a wrong
/// "could not ask" only asks him to try again, while a wrong "never heard of
/// it" retires a pair he has drawn months of levels on.
///
/// 200 is *"No security definition has been found for the request"* — IBKR
/// looked, and there is nothing there.
const NO_SUCH_THING: [i32; 1] = [200];

/// Does this code mean IBKR looked and found nothing?
pub(super) fn never_heard_of_it(code: i32) -> bool {
    NO_SUCH_THING.contains(&code)
}
