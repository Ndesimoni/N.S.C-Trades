//! Driving Chrome, and cleaning up after it.

use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use super::CardError;

const CHROME: &str = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";

/// How wide every card is.
const WIDTH: u32 = 860;

/// Twice the pixels, so the text is still sharp when a phone scales it up.
const SCALE: &str = "--force-device-scale-factor=2";

/// Fast-forwards the page's clock. Without it the screenshot catches the
/// animations halfway and the fonts before they have arrived.
const SETTLE: &str = "--virtual-time-budget=4000";

/// How long Chrome gets before it is stopped.
///
/// **A card takes about two seconds.** A minute is far past generous, and it
/// is not "forever" — which is what it had, and what wedged the whole bot: a
/// Chrome that draws the picture and then never exits leaves the call waiting
/// on it waiting for good. Nothing answered, not even `/help`, and nothing
/// said why.
const PATIENCE: Duration = Duration::from_secs(60);

/// How often to look and see whether Chrome has finished.
const GLANCE: Duration = Duration::from_millis(100);

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

/// Waits for Chrome, but not forever.
///
/// **`output()` waits for good**, and Chrome can finish its work and then not
/// exit. When that happened the bot stopped answering anything at all — the
/// thread was stuck in here, and nothing in any log said so.
///
/// Killed rather than left, because a Chrome that will not exit is one more
/// every time a card is drawn.
fn wait_for(mut chrome: std::process::Child) -> Result<String, CardError> {
    let give_up_at = Instant::now() + PATIENCE;

    loop {
        match chrome.try_wait() {
            Err(trouble) => return Err(CardError::DrewNothing(trouble.to_string())),

            // Finished on its own. Whatever it said is the reason if no
            // picture appeared.
            Ok(Some(_)) => {
                let mut said = String::new();

                if let Some(mut errors) = chrome.stderr.take() {
                    use std::io::Read;
                    let _ = errors.read_to_string(&mut said);
                }

                return Ok(said);
            }

            Ok(None) if Instant::now() >= give_up_at => {
                let _ = chrome.kill();
                let _ = chrome.wait();

                return Err(CardError::DrewNothing(format!(
                    "Chrome had not finished after {} seconds and was stopped",
                    PATIENCE.as_secs()
                )));
            }

            Ok(None) => std::thread::sleep(GLANCE),
        }
    }
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
