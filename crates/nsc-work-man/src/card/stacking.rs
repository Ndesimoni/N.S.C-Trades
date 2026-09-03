//! **Three pictures into one**, so the buttons can sit under all of them.
//!
//! ## Why this exists
//!
//! A setup is three pictures — the run, the close-up, the card. They used to
//! go as a group of photos, and **Telegram does not allow buttons on a group
//! of photos.** So the tick and the cross arrived in a message of their own,
//! which he did not want: *"they are in a different card."*
//!
//! Splitting the group so the card could carry them was closer, and still not
//! it: *"it should ALL be on one card — the 200 run, the 45, and then the
//! final setup, and the take or the skip under."*
//!
//! There is exactly one way Telegram allows that, and this is it. One image,
//! one message, buttons underneath.
//!
//! ## What it costs, so nobody is surprised
//!
//! The result is tall — three cards stacked — so the chat feed shows it small
//! until he taps it. That is the trade for having everything in one place with
//! the buttons attached, and it is the trade he asked for.

use std::path::{Path, PathBuf};

use image::{GenericImage, RgbaImage};

use super::CardError;

/// The join between two stacked cards, in pixels.
///
/// **Enough to read as a seam, not as a gap.** Butted straight together, the
/// bottom row of one card and the top row of the next look like one picture
/// that has gone wrong.
const SEAM: u32 = 8;

/// The colour of that seam — the same off-white the cards sit on.
const PAPER: [u8; 4] = [0xF5, 0xF7, 0xF8, 0xFF];

/// Stacks pictures top to bottom into one, in the order given.
///
/// **Widest wins and the rest are left-aligned.** In practice every card comes
/// out of the same 860-point page so they already match; centring code for a
/// case that cannot happen is code nobody will ever check.
pub fn stack(parts: &[&Path], out: &Path) -> Result<PathBuf, CardError> {
    let mut loaded = Vec::with_capacity(parts.len());

    for part in parts {
        let picture = image::open(part).map_err(|trouble| CardError::CannotWrite {
            path: part.display().to_string(),
            detail: format!("could not read it back: {trouble}"),
        })?;

        loaded.push(picture.to_rgba8());
    }

    let Some(width) = loaded.iter().map(|one| one.width()).max() else {
        return Err(CardError::NothingToDraw);
    };

    let seams = SEAM * (loaded.len().saturating_sub(1)) as u32;
    let height = loaded.iter().map(|one| one.height()).sum::<u32>() + seams;

    let mut sheet = RgbaImage::from_pixel(width, height, image::Rgba(PAPER));
    let mut top = 0;

    for one in &loaded {
        // **A copy that cannot fail is still checked.** It only fails when the
        // source would fall outside the sheet, which the sums above rule out —
        // and a silent `let _` is how a wrong sum becomes a blank picture.
        sheet
            .copy_from(one, 0, top)
            .map_err(|trouble| CardError::DrewNothing(trouble.to_string()))?;

        top += one.height() + SEAM;
    }

    sheet.save(out).map_err(|trouble| CardError::CannotWrite {
        path: out.display().to_string(),
        detail: trouble.to_string(),
    })?;

    Ok(out.to_path_buf())
}
