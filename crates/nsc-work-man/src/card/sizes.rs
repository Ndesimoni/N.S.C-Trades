//! **How many candles a chart draws.**
//!
//! Two numbers, and they live here rather than at the four places that draw a
//! chart. Each of those used to slice for itself, so one of them did not —
//! `review/picture.rs` drew whatever the feed returned, which on the hourly is
//! over three hundred. He spotted it on AUD/USD, 1 September 2026.
//!
//! `card::render` cuts to [`RUN`] and `card::render_ringed` to [`CONTEXT`], so
//! there is nothing for a caller to remember.

/// How many candles **the run** shows — the widest of the three pictures.
///
/// **His ask, 30 August 2026:** *"so that I see the direction the price has
/// been coming from... if it's coming from down to up, or if it's doing a
/// curve, or if it's going from up to down."*
///
/// **Cut from four hundred on 1 September 2026**, the same day and for the
/// same reason as the close-up: *"reduce the candles in the first chart too,
/// so we can see it clear."*
///
/// At four hundred the bodies were on their floor — 1.5 units, about 3px — so
/// the picture was a texture rather than candles. Two hundred gives 2.0
/// units, about 3.8px, and you can tell one candle from the next.
///
/// **It still shows the whole move**, which is its only job: on the AUD/USD
/// hourly it is 8 days, and that carried the drop, the base, the push up, the
/// top and the pull back into the level with room to spare.
///
/// **Two hundred since 1 September 2026**, his number: *"for the empty run
/// let it be 200 candles, not 150."* The empty run is this one — the wide
/// chart with no ring on it. About 11 days on the 1-hour, 33 on the 4-hour,
/// 9 months on the daily.
pub const RUN: usize = 200;

/// How many candles **the close-up** shows, the one carrying the red ring.
///
/// **Cut from a hundred on 1 September 2026:** *"even when I zoom into the
/// picture I still do not see the setup clearly."*
///
/// A hundred was his own number a few days earlier, and it was the right
/// answer to the question he was asking then — *"so I can see what played out,
/// how it played out."* It was the wrong number for SEEING it.
///
/// **The candles are drawn to fit, so the count IS the size.** The plot is 728
/// units wide and 82% of that is drawn, which is 597 for however many candles
/// there are:
///
/// ```text
///     100 candles    body  3.6 units   wick 0.6    ~7px and ~1px on the phone
///      45 candles    body  9.0 units   wick 2.0   ~17px and ~4px
/// ```
///
/// **The wick was the half that broke it.** At a hundred candles it came out
/// under a pixel, and a pin bar IS its wick — so the one shape that most needs
/// to be seen was the one being rounded away. Zooming a picture cannot put
/// back a line that was never drawn.
///
/// Forty-five is about 2 days on the 1-hour, 7.5 on the 4-hour and 9 weeks on
/// the daily. **The run picture still carries the history**; this one only has
/// to show the shape and what walked into the level.
pub const CONTEXT: usize = 45;
