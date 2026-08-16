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

/// **A profile of its own, for this one drawing.**
///
/// Chrome refuses to start on a profile another Chrome is holding — it says
/// "Failed to create a ProcessSingleton for your profile directory" and gives
/// up. With no `--user-data-dir` it reaches for the DEFAULT profile, which is
/// the one his own open browser is holding, so cards failed whenever Chrome
/// was open and worked whenever it was not. Intermittent, and nothing in the
/// message said why.
///
/// **One fixed folder is not enough either.** The bot draws cards while
/// `--bin cards` is drawing one, and the watcher draws while the inbox
/// answers `/status`. Any shared folder is a lock two of them can want.
///
/// So: a fresh one per drawing, thrown away after. Chrome builds it in about
/// a second, which for a card sent every few minutes is nothing.
fn own_profile() -> std::path::PathBuf {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_nanos())
        .unwrap_or_default();

    std::env::temp_dir().join(format!("{LEFTOVER}{}-{stamp}", std::process::id()))
}

/// What every throwaway profile is called, so the stale ones can be found.
const LEFTOVER: &str = "nsc-chrome-";

/// How long a profile has to sit there before it counts as abandoned.
const ABANDONED: std::time::Duration = std::time::Duration::from_secs(3600);

/// **Clears out profiles nobody is using.**
///
/// Deleting one the moment Chrome exits is not enough on its own: Chrome
/// leaves helper processes running for a moment after the one we waited on has
/// gone, and they build the folder straight back. One card every few minutes
/// for a fortnight is a lot of folders.
///
/// So anything of ours older than an hour goes. An hour is far longer than a
/// drawing takes, so this can never delete one in use — including one another
/// copy of the bot is using right now.
///
/// Failing to tidy up is never worth stopping for. It is housekeeping.
fn sweep_old_profiles() {
    let Ok(entries) = std::fs::read_dir(std::env::temp_dir()) else {
        return;
    };

    for old in entries.flatten() {
        if !old.file_name().to_string_lossy().starts_with(LEFTOVER) {
            continue;
        }

        let sat_there = old
            .metadata()
            .and_then(|about| about.modified())
            .map(|at| at.elapsed().unwrap_or_default());

        if sat_there.is_ok_and(|since| since > ABANDONED) {
            let _ = std::fs::remove_dir_all(old.path());
        }
    }
}

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

    sweep_old_profiles();
    let profile = own_profile();

    let done = Command::new(CHROME)
        .args([
            "--headless",
            "--disable-gpu",
            "--hide-scrollbars",
            SCALE,
            SETTLE,
            &format!("--user-data-dir={}", profile.display()),
            &format!("--window-size={WIDTH},{}", height + RESERVED),
            &format!("--screenshot={}", out.display()),
            &format!("file://{}", page.display()),
        ])
        .output();

    // Thrown away whether it worked or not. Left behind, they pile up in the
    // temp folder for as long as the bot runs.
    let _ = std::fs::remove_dir_all(&profile);

    let done = done.map_err(|trouble| CardError::DrewNothing(trouble.to_string()))?;

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
