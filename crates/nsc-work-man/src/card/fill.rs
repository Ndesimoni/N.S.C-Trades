//! Putting real numbers into a template and asking for a picture of it.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use super::{chrome, facts};
use crate::candle::Bar;
use crate::levels::Band;

/// Where the templates live.
const TEMPLATES: &str = "assets/card";

/// Fills in a template and screenshots it.
///
/// `template` is a file in `assets/card/`. `bars` are the finished candles,
/// oldest first — the newest of them is the one the card describes.
///
/// Gives back the absolute path of the picture.
/// `interval` is the timeframe these candles are, in the feed's own spelling —
/// `1h`, `1week`. The card says it out loud, so it has to be told rather than
/// assume: the levels chart is weekly and spent a render claiming to be hourly.
pub fn render(
    template: &str,
    bars: &[&Bar],
    bands: &[Band],
    interval: &str,
    digits: u32,
    out: &Path,
) -> Result<PathBuf> {
    let Some(latest) = bars.last() else {
        bail!("there are no candles to draw");
    };

    let source = Path::new(TEMPLATES).join(template);
    let html = std::fs::read_to_string(&source)
        .with_context(|| format!("could not read the card template at {}", source.display()))?;

    let height = height_of(&html)
        .with_context(|| format!("{template} has no --card-height line in its CSS"))?;

    let filled = html
        .replace(
            "/*__CANDLE__*/",
            &facts::one(latest, interval, digits)?.to_string(),
        )
        .replace("/*__BARS__*/", &facts::all(bars, digits).to_string())
        .replace("/*__LEVELS__*/", &facts::levels(bands, digits).to_string());

    // The page is written next to the picture, not into a temp folder. Open it
    // in a browser and the card is there with real numbers in it — edit the
    // template, refresh, see the change. That loop is the whole point of the
    // design living in HTML.
    let page = out.with_extension("html");
    make_room_for(&page)?;
    std::fs::write(&page, filled).context("could not write the filled-in template")?;
    make_room_for(out)?;

    // Absolute, both of them, always. Chrome runs with its own working folder,
    // so `file://preview/chart.html` makes it read `preview` as a HOSTNAME —
    // and it quietly screenshots its own error page, which then goes out
    // looking like a real card.
    let page = std::fs::canonicalize(&page)
        .with_context(|| format!("could not resolve {}", page.display()))?;

    let picture =
        std::path::absolute(out).with_context(|| format!("could not resolve {}", out.display()))?;

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

fn make_room_for(file: &Path) -> Result<()> {
    let Some(folder) = file.parent() else {
        return Ok(());
    };

    std::fs::create_dir_all(folder).with_context(|| format!("could not make {}", folder.display()))
}
