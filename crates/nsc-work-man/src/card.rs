//! Rendering a card by filling in an HTML template and letting Chrome draw it.
//!
//! The design lives in `assets/card/*.html`, not in Rust. He can open that file,
//! change it, and the next message picks it up — no rebuild, no code.
//!
//! **The cost:** whatever machine runs this needs Chrome installed. Fine on a
//! Mac. On a server it is a real dependency, and worth remembering before this
//! goes anywhere but a laptop.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use chrono::TimeDelta;
use rust_decimal::Decimal;
use serde_json::json;

use crate::candle::Bar;
use crate::settings::{INTERVAL, INTERVAL_MINUTES, SYMBOL, timeframe_name, unit_for};

const CHROME: &str = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";

/// Twice the pixels, so the text is still sharp when a phone scales it up.
const SCALE: &str = "--force-device-scale-factor=2";

/// Fast-forwards the page's clock. Without it the screenshot catches the
/// animations halfway and the fonts before they have arrived.
const SETTLE: &str = "--virtual-time-budget=4000";

/// How much shorter the page is than the window Chrome was asked for.
///
/// Measured, not guessed: ask for 600 and the page gets 513, ask for 900 and it
/// gets 813. Always 87, and always painted white at the bottom of the
/// screenshot.
///
/// The old headless mode did not do this. It has been removed from Chrome, so
/// the strip is asked for and then cut off.
const CHROME_RESERVES: u32 = 87;

/// Twice the pixels means twice the strip.
const PIXELS_PER_POINT: u32 = 2;

/// Fills in a template and screenshots it.
///
/// `template` is a file in `assets/card/`. `bars` are the finished candles,
/// oldest first. The newest of them is the one the readout describes.
///
/// How tall the picture is comes out of the template itself — the
/// `--card-height` line in its CSS. Chrome screenshots a window rather than a
/// page, so something has to say how tall, and the file being designed is the
/// honest place for it. Two numbers in two files drift apart; one does not.
pub fn render(template: &str, bars: &[&Bar], digits: u32, out: &Path) -> Result<PathBuf> {
    let Some(latest) = bars.last() else {
        bail!("there are no candles to draw");
    };

    // Chrome is given absolute paths for both the page and the picture. It
    // runs with its own working folder, so a relative one is either read as a
    // hostname or written somewhere nobody looks.
    if let Some(folder) = out.parent() {
        std::fs::create_dir_all(folder)
            .with_context(|| format!("could not make {}", folder.display()))?;
    }

    let source = Path::new("assets/card").join(template);
    let html = std::fs::read_to_string(&source)
        .with_context(|| format!("could not read the card template at {}", source.display()))?;

    let filled = html
        .replace("/*__CANDLE__*/", &describe(latest, digits)?.to_string())
        .replace("/*__BARS__*/", &series(bars, digits).to_string());

    let height = card_height(&html)
        .with_context(|| format!("{template} has no --card-height line in its CSS"))?;

    // Next to the picture, not in a temp folder. Open it in a browser and the
    // card is there with real numbers in it — edit the template, refresh, see
    // the change. That loop is the whole point of the design living in HTML.
    let page = out.with_extension("html");
    if let Some(folder) = page.parent() {
        std::fs::create_dir_all(folder)
            .with_context(|| format!("could not make {}", folder.display()))?;
    }
    std::fs::write(&page, filled).context("could not write the filled-in template")?;

    // Absolute, always. `file://preview/chart.html` makes Chrome read
    // `preview` as a hostname and it quietly screenshots its own error page —
    // which then goes to Telegram looking like a real card.
    let page = std::fs::canonicalize(&page)
        .with_context(|| format!("could not resolve {}", page.display()))?;

    let picture = std::path::absolute(out)
        .with_context(|| format!("could not resolve {}", out.display()))?;

    shoot(&page, height, &picture)?;

    Ok(picture)
}

/// The one candle the readout is about.
fn describe(bar: &Bar, digits: u32) -> Result<serde_json::Value> {
    let one_step = TimeDelta::try_minutes(INTERVAL_MINUTES)
        .context("the interval is not a length of time chrono can hold")?;

    let closed_at = bar.opened_at()? + one_step;
    let number = |value: Decimal| {
        value
            .round_dp(digits)
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0)
    };

    Ok(json!({
        "symbol":   SYMBOL,
        "interval": timeframe_name(INTERVAL),
        "stamp":    closed_at.format("%-d %b · %H:%M UTC").to_string(),
        "unit":     unit_for(SYMBOL),
        "digits":   digits,
        "open":     number(bar.open),
        "high":     number(bar.high),
        "low":      number(bar.low),
        "close":    number(bar.close),
    }))
}

/// Every candle, for the chart. Oldest first, because that is how a chart reads.
fn series(bars: &[&Bar], digits: u32) -> serde_json::Value {
    let number = |value: Decimal| {
        value
            .round_dp(digits)
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0)
    };

    let rows: Vec<serde_json::Value> = bars
        .iter()
        .map(|bar| {
            json!({
                "at":    bar.datetime.get(5..16).unwrap_or(&bar.datetime),
                "open":  number(bar.open),
                "high":  number(bar.high),
                "low":   number(bar.low),
                "close": number(bar.close),
            })
        })
        .collect();

    json!(rows)
}

/// Chrome, headless, one screenshot.
fn shoot(page: &Path, height: u32, out: &Path) -> Result<()> {
    if !Path::new(CHROME).exists() {
        bail!("Chrome is not at {CHROME}, and the card is drawn by Chrome");
    }

    let done = Command::new(CHROME)
        .args([
            "--headless",
            "--disable-gpu",
            "--hide-scrollbars",
            SCALE,
            SETTLE,
            &format!("--window-size=860,{}", height + CHROME_RESERVES),
            &format!("--screenshot={}", out.display()),
            &format!("file://{}", page.display()),
        ])
        .output()
        .context("could not start Chrome")?;

    if !out.exists() {
        bail!(
            "Chrome ran but wrote no picture:\n{}",
            String::from_utf8_lossy(&done.stderr)
        );
    }

    trim(out, height * PIXELS_PER_POINT)
}

/// Cuts the white strip off the bottom.
fn trim(picture: &Path, keep: u32) -> Result<()> {
    let shot =
        image::open(picture).with_context(|| format!("could not open {}", picture.display()))?;

    let wide = shot.width();
    if shot.height() <= keep {
        return Ok(());
    }

    image::imageops::crop_imm(&shot, 0, 0, wide, keep)
        .to_image()
        .save(picture)
        .with_context(|| format!("could not save the trimmed {}", picture.display()))?;

    Ok(())
}

/// Pulls `--card-height:748px;` out of the template's CSS.
fn card_height(html: &str) -> Option<u32> {
    let after = html.split("--card-height:").nth(1)?;
    let digits: String = after
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();

    digits.parse().ok()
}
