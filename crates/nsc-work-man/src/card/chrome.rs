//! Driving Chrome, and cleaning up after it.

use std::path::Path;
use std::process::{Command, Stdio};

use super::CardError;
use super::waiting::wait_for;

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
///
/// **This blocks, and it is called from async code.** For the two to ten
/// seconds Chrome takes, one of Tokio's worker threads does nothing but wait
/// here. On his Mac there are eight of them and prices keep arriving on the
/// others; on a one-core box the whole bot stops until the card is drawn.
///
/// **Fixed 30 August 2026:** all ten places that draw a card go through
/// `tokio::task::spawn_blocking`, so no worker thread ever waits on Chrome.
/// What follows is the reasoning, kept because it is why the wrapping exists.
///
/// The fix is `spawn_blocking` at the places that draw a card, which needs
/// their inputs owned rather than borrowed. It is the top item in
/// `PROGRESS.md`. Do not add a seventh caller without reading that first.
pub fn shoot(page: &Path, height: u32, out: &Path) -> Result<(), CardError> {
    if !Path::new(CHROME).exists() {
        return Err(CardError::NoChrome(CHROME.into()));
    }

    clear_the_way(out)?;

    // **No `--user-data-dir`, and that is deliberate.**
    //
    // Left alone, headless Chrome makes its own throwaway profile, draws, and
    // exits in about two seconds. Point it at a folder of our own and it
    // writes the picture and then NEVER EXITS — so the call waiting on it
    // waits for good, and the bot stops answering anything at all.
    //
    // That was tried, as a fix for a clash that turned out not to exist. The
    // real clash was two copies of the bot writing the same card file. See
    // inbox/hearing.rs.
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
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn();

    let done = wait_for(done.map_err(|trouble| CardError::DrewNothing(trouble.to_string()))?)?;

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
        return Err(CardError::DrewNothing(done));
    }

    trim(out, height * PIXELS_PER_POINT)
}

/// Takes the last picture of this kind out of the way.
///
/// **A file appearing is the only check that Chrome drew anything** — and one
/// was already there, left by the last card of the same kind. Chrome fails,
/// the old picture survives the check, and it goes out with today's caption on
/// yesterday's chart.
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
    // **Not a write failure.** Chrome can leave a file that is not a picture —
    // half of one, if it was stopped part way. Reported as "could not write"
    // it read like a full disk, and an hour went on the wrong thing.
    let shot = image::open(picture).map_err(|trouble| {
        CardError::DrewNothing(format!(
            "Chrome left something at {} that is not a picture: {trouble}",
            picture.display()
        ))
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
