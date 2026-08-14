//! Driving Chrome, and cleaning up after it.

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

const CHROME: &str = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";

/// How wide every card is.
const WIDTH: u32 = 860;

/// Twice the pixels, so the text is still sharp when a phone scales it up.
const SCALE: &str = "--force-device-scale-factor=2";

/// Fast-forwards the page's clock. Without it the screenshot catches the
/// animations halfway and the fonts before they have arrived.
const SETTLE: &str = "--virtual-time-budget=4000";

/// How much shorter the page is than the window Chrome was asked for.
///
/// **Measured, not guessed:** ask for 600 and the page gets 513, ask for 900
/// and it gets 813. Always 87 — and Chrome paints that strip white at the
/// bottom of the screenshot.
///
/// The old headless mode did not do this. It has been removed from Chrome, so
/// the strip is asked for and then cut off.
const RESERVED: u32 = 87;

/// Twice the pixels means twice the strip.
const PIXELS_PER_POINT: u32 = 2;

/// One screenshot of one page.
///
/// Both paths must already be absolute — see the note in `fill.rs` about what
/// happens when they are not.
pub fn shoot(page: &Path, height: u32, out: &Path) -> Result<()> {
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
            &format!("--window-size={WIDTH},{}", height + RESERVED),
            &format!("--screenshot={}", out.display()),
            &format!("file://{}", page.display()),
        ])
        .output()
        .context("could not start Chrome")?;

    // Chrome answers 0 whether it drew the card or its own error page, so the
    // only honest check is whether a file appeared.
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

    if shot.height() <= keep {
        return Ok(());
    }

    image::imageops::crop_imm(&shot, 0, 0, shot.width(), keep)
        .to_image()
        .save(picture)
        .with_context(|| format!("could not save the trimmed {}", picture.display()))?;

    Ok(())
}
