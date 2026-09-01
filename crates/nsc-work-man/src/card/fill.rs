//! Putting real numbers into a template and asking for a picture of it.

use std::path::{Path, PathBuf};

use super::CardError;

use super::{chrome, facts};
use nsc_core::candle::Bar;
use nsc_core::levels::Band;

/// Where the templates live.
const TEMPLATES: &str = "assets/card";

/// The styling every card shares — the palette, the typefaces, the page box.
///
/// A card's own styling sits beside its template as `<name>.css` and goes in
/// where the template says `__OWN__`. Missing is not an error: a card with
/// nothing of its own simply has none.
///
/// Dropped in where a template says `__STYLE__`, so a colour is changed in one
/// place and every card follows. Inlined rather than linked because the filled
/// page is written next to the picture, and a link would break the moment the
/// two are not in the same folder.
const STYLE: &str = "style.css";

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

/// The last `many` candles, or all of them when there are fewer.
pub(super) fn newest<'a>(bars: &[&'a Bar], many: usize) -> Vec<&'a Bar> {
    bars[bars.len().saturating_sub(many)..].to_vec()
}

/// Fills in a candle template and screenshots it.
///
/// `template` is a file in `assets/card/`. `bars` are the finished candles,
/// oldest first — the newest of them is the one the card describes.
///
/// Gives back the absolute path of the picture.
/// **The card is told everything it says out loud.** It reads no constants.
///
/// It assumed both at different times and lied about both — a weekly chart
/// headed `1 HOUR`, and GBPUSD candles headed `XAU/USD`. A picture with the
/// wrong name on it is believed exactly like a wrong number.
///
/// `interval` is the feed's own spelling — `1h`, `1week`.
/// **THE WIDE ONE. It draws [`RUN`] candles and never more**, whatever it is
/// handed.
///
/// The cap is here rather than at the call sites, and that is the fix of
/// 1 September 2026. There are four callers and each was slicing for itself —
/// so one of them did not. `review/picture.rs`, the chart he gets when he ASKS
/// for one, drew whatever the fetch returned: it asks IBKR for 150 candles,
/// IBKR reads that as a span of days, and 14 days of hourly forex came back as
/// **over three hundred**. His words: *"why is AUDUSD showing me 300+ candles
/// — for all pairs we need only 200."*
///
/// A rule every caller has to remember is a rule one of them will forget. Now
/// there is nothing to remember.
pub fn render(
    template: &str,
    bars: &[&Bar],
    bands: &[Band],
    symbol: &str,
    interval: &str,
    digits: u32,
    out: &Path,
) -> Result<PathBuf, CardError> {
    chart(
        template,
        &newest(bars, RUN),
        bands,
        symbol,
        interval,
        digits,
        None,
        out,
    )
}

/// The same chart, with the last `ring` candles circled in red.
///
/// **The shape a signal is made of always finishes on the newest candle**, so a
/// count is all this needs and there is no index to get wrong.
///
/// `None` draws no ring, which is what `/chart` wants — that picture answers
/// "where is price", not "look here".
#[allow(clippy::too_many_arguments)]
/// **THE CLOSE-UP. It draws [`CONTEXT`] candles and never more.** Same cap in
/// the same place, for the same reason as [`render`].
#[allow(clippy::too_many_arguments)]
pub fn render_ringed(
    template: &str,
    bars: &[&Bar],
    bands: &[Band],
    symbol: &str,
    interval: &str,
    digits: u32,
    ring: Option<usize>,
    out: &Path,
) -> Result<PathBuf, CardError> {
    chart(
        template,
        &newest(bars, CONTEXT),
        bands,
        symbol,
        interval,
        digits,
        ring,
        out,
    )
}

/// Fills the template in and shoots it. **Both of the two above come through
/// here, already cut to size.**
#[allow(clippy::too_many_arguments)]
fn chart(
    template: &str,
    bars: &[&Bar],
    bands: &[Band],
    symbol: &str,
    interval: &str,
    digits: u32,
    ring: Option<usize>,
    out: &Path,
) -> Result<PathBuf, CardError> {
    let Some(latest) = bars.last() else {
        return Err(CardError::NothingToDraw);
    };

    draw(
        template,
        &[
            (
                "/*__CANDLE__*/",
                facts::one(latest, symbol, interval, digits)?.to_string(),
            ),
            ("/*__BARS__*/", facts::all(bars, digits).to_string()),
            ("/*__LEVELS__*/", facts::levels(bands, digits).to_string()),
            (
                "/*__RING__*/",
                ring.map_or_else(|| "null".to_string(), |many| many.to_string()),
            ),
        ],
        out,
    )
}

/// Reads a template, puts the facts in, and screenshots the result.
///
/// Every card goes through here. `fills` are the marker each template leaves
/// for its own facts and what to put there.
pub(super) fn draw(
    template: &str,
    fills: &[(&str, String)],
    out: &Path,
) -> Result<PathBuf, CardError> {
    let source = Path::new(TEMPLATES).join(template);
    let html = std::fs::read_to_string(&source).map_err(|trouble| CardError::NoTemplate {
        path: source.display().to_string(),
        detail: trouble.to_string(),
    })?;

    let style = std::fs::read_to_string(Path::new(TEMPLATES).join(STYLE)).map_err(|trouble| {
        CardError::NoTemplate {
            path: STYLE.into(),
            detail: trouble.to_string(),
        }
    })?;

    // The card's own styling, from a file beside the template. It keeps the
    // HTML a thing that can be read end to end rather than scrolled — the
    // markup and the script are what change; the CSS mostly sits still.
    let own = template.replace(".html", ".css");
    let own = std::fs::read_to_string(Path::new(TEMPLATES).join(&own)).unwrap_or_default();

    let mut filled = html
        .replace("/*__STYLE__*/", &style)
        .replace("/*__OWN__*/", &own);

    for (marker, value) in fills {
        filled = filled.replace(marker, value);
    }

    // **After the facts go in, not before.** A card whose height depends on
    // what it is showing — the heartbeat lists a row per pair — writes its
    // `--card-height` as one of those facts. Read it any earlier and Rust asks
    // Chrome for a window the page never agreed to, and the difference comes
    // out as a strip of white or a clipped last row.
    let height = height_of(&filled).ok_or_else(|| CardError::NoHeight(template.into()))?;

    // The page is written next to the picture, not into a temp folder. Open it
    // in a browser and the card is there with real numbers in it — edit the
    // template, refresh, see the change. That loop is the whole point of the
    // design living in HTML.
    let page = out.with_extension("html");
    make_room_for(&page)?;
    std::fs::write(&page, filled).map_err(|trouble| CardError::CannotWrite {
        path: page.display().to_string(),
        detail: trouble.to_string(),
    })?;
    make_room_for(out)?;

    // Absolute, both of them, always. Chrome runs with its own working folder,
    // so `file://preview/chart.html` makes it read `preview` as a HOSTNAME —
    // and it quietly screenshots its own error page, which then goes out
    // looking like a real card.
    let page = std::fs::canonicalize(&page).map_err(|trouble| CardError::CannotWrite {
        path: page.display().to_string(),
        detail: trouble.to_string(),
    })?;

    let picture = std::path::absolute(out).map_err(|trouble| CardError::CannotWrite {
        path: out.display().to_string(),
        detail: trouble.to_string(),
    })?;

    chrome::shoot(&page, height, &picture)?;

    Ok(picture)
}

/// Pulls `--card-height:748px;` out of the template's own CSS.
///
/// Chrome screenshots a **window**, not a page, so something has to say how
/// tall. The file being designed is the honest place for it — two numbers in
/// two files drift apart, one does not.
///
/// **The last one wins, because that is what the browser does.** `style.css`
/// sets a shared height and is dropped in at the top, so a card that wants its
/// own says so further down and both Chrome and this agree on which. Reading
/// the first would leave Rust asking for one height while the page drew
/// another, and the difference comes out as a strip of white.
pub fn height_of(html: &str) -> Option<u32> {
    // rsplit always yields something. If the text was never there it is the
    // whole file, which starts with `<` and parses to nothing.
    let after = html.rsplit("--card-height:").next()?;

    let digits: String = after
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();

    digits.parse().ok()
}

fn make_room_for(file: &Path) -> Result<(), CardError> {
    let Some(folder) = file.parent() else {
        return Ok(());
    };

    std::fs::create_dir_all(folder).map_err(|trouble| CardError::CannotWrite {
        path: folder.display().to_string(),
        detail: trouble.to_string(),
    })
}
