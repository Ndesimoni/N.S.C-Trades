//! Putting real numbers into a template and asking for a picture of it.

use std::path::{Path, PathBuf};

use nsc_core::error::CardError;

use super::{chrome, facts};
use nsc_core::candle::Bar;
use nsc_core::levels::Band;

/// Where the templates live.
const TEMPLATES: &str = "assets/card";

/// The styling every card shares — the palette, the typefaces, the page box.
///
/// Dropped in where a template says `__STYLE__`, so a colour is changed in one
/// place and every card follows. Inlined rather than linked because the filled
/// page is written next to the picture, and a link would break the moment the
/// two are not in the same folder.
const STYLE: &str = "style.css";

/// Fills in a template and screenshots it.
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
pub fn render(
    template: &str,
    bars: &[&Bar],
    bands: &[Band],
    symbol: &str,
    interval: &str,
    digits: u32,
    out: &Path,
) -> Result<PathBuf, CardError> {
    let Some(latest) = bars.last() else {
        return Err(CardError::NothingToDraw);
    };

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

    let html = html.replace("/*__STYLE__*/", &style);

    let height = height_of(&html).ok_or_else(|| CardError::NoHeight(template.into()))?;

    let filled = html
        .replace(
            "/*__CANDLE__*/",
            &facts::one(latest, symbol, interval, digits)?.to_string(),
        )
        .replace("/*__BARS__*/", &facts::all(bars, digits).to_string())
        .replace("/*__LEVELS__*/", &facts::levels(bands, digits).to_string());

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
pub fn height_of(html: &str) -> Option<u32> {
    let after = html.split("--card-height:").nth(1)?;

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
