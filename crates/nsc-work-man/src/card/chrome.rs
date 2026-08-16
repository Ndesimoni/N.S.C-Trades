//! Driving Chrome, and cleaning up after it.

use std::path::Path;
use std::process::Command;

use super::CardError;

const CHROME: &str = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";

/// How wide every card is.
const WIDTH: u32 = 860;

/// Twice the pixels, so the text is still sharp when a phone scales it up.
const SCALE: &str = "--force-device-scale-factor=2";

/// Fast-forwards the page's clock. Without it the screenshot catches the
/// animations halfway and the fonts before they have arrived.
const SETTLE: &str = "--virtual-time-budget=4000";

/// **Its own profile, so it never fights the browser he is using.**
///
/// Without this, Chrome reaches for the default profile — the one his own
/// open browser is holding. It then exits without drawing, having already
/// created the file, so the "did a picture appear?" check passed on nought
/// bytes and the image reader failed with "unexpected end of file".
///
/// It happened intermittently, which is worse than always: `/status` worked
/// when his browser was shut and failed when it was open, and nothing in the
/// message said why.
///
/// Under `preview/`, which is already ignored by git.
const PROFILE: &str = "preview/.chrome";

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
pub fn shoot(page: &Path, height: u32, out: &Path) -> Result<(), CardError> {
    if !Path::new(CHROME).exists() {
        return Err(CardError::NoChrome(CHROME.into()));
    }

    clear_the_way(out)?;

    let done = Command::new(CHROME)
        .args([
            "--headless",
            "--disable-gpu",
            "--hide-scrollbars",
            SCALE,
            SETTLE,
            &format!("--user-data-dir={PROFILE}"),
            &format!("--window-size={WIDTH},{}", height + RESERVED),
            &format!("--screenshot={}", out.display()),
            &format!("file://{}", page.display()),
        ])
        .output()
        .map_err(|trouble| CardError::DrewNothing(trouble.to_string()))?;

    // **Chrome answers 0 whether it drew the card, its own error page, or
    // nothing at all** — so the only honest check is the file itself.
    //
    // `exists` was that check and it was not enough. Chrome creates the file
    // before it writes to it, so a run that gave up left nought bytes behind
    // and this said the card was drawn. What came back was the image reader
    // failing on an empty file — "unexpected end of file" — which reads like a
    // disk fault rather than what it was.
    let drawn = std::fs::metadata(out).map(|about| about.len()).unwrap_or(0);

    if drawn == 0 {
        return Err(CardError::DrewNothing(
            String::from_utf8_lossy(&done.stderr).into_owned(),
        ));
    }

    trim(out, height * PIXELS_PER_POINT)
}

/// Takes the last picture of this kind out of the way.
///
/// **The only check that Chrome drew anything is whether a file appeared** —
/// and one was already there, left by the last card of the same kind. Chrome
/// fails, the old picture survives the check, and it goes out with today's
/// caption on yesterday's chart.
///
/// Nothing to remove is fine. Being unable to remove one that IS there has to
/// stop here, because carrying on would send the stale picture.
pub(super) fn clear_the_way(out: &Path) -> Result<(), CardError> {
    match std::fs::remove_file(out) {
        Ok(()) => Ok(()),
        Err(trouble) if trouble.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(trouble) => Err(CardError::CannotWrite {
            path: out.display().to_string(),
            detail: trouble.to_string(),
        }),
    }
}

/// Cuts the white strip off the bottom.
fn trim(picture: &Path, keep: u32) -> Result<(), CardError> {
    let shot = image::open(picture).map_err(|trouble| CardError::CannotWrite {
        path: picture.display().to_string(),
        detail: trouble.to_string(),
    })?;

    if shot.height() <= keep {
        return Ok(());
    }

    image::imageops::crop_imm(&shot, 0, 0, shot.width(), keep)
        .to_image()
        .save(picture)
        .map_err(|trouble| CardError::CannotWrite {
            path: picture.display().to_string(),
            detail: trouble.to_string(),
        })?;

    Ok(())
}
