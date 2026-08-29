//! Saying what the scan found — to the terminal, and to Telegram.

use anyhow::{Context, Result};
use nsc_strategy::reasons;
use nsc_work_man::places::OWNER;
use nsc_work_man::telegram;

use super::Found;

/// The whole scan, on screen.
pub fn to_terminal(found: &[Found]) {
    let zones: usize = found.iter().map(|one| one.bands.len()).sum();
    let hits: usize = found.iter().map(|one| one.at_zones.len()).sum();

    println!("\n══════ what is at your zones ══════\n");
    println!(
        "  {} pair-and-timeframe readings · {zones} zone checks · {hits} at a zone\n",
        found.len()
    );

    for one in found {
        let shapes = if one.shapes.is_empty() {
            "no shapes".to_string()
        } else {
            format!("{} shape(s)", one.shapes.len())
        };

        println!(
            "  {:9} {:5}  last {:>12}  {} · {} zone(s)",
            one.symbol,
            one.interval.spoken(),
            one.last.close.round_dp(one.digits).to_string(),
            shapes,
            one.bands.len(),
        );

        for (shape, when) in one.shapes.iter().rev().take(3) {
            println!("             ↳ {shape} on {when}");
        }

        for signal in &one.at_zones {
            println!(
                "             ★ {}",
                reasons::sentence(signal, &one.symbol, one.interval.spoken(), one.digits)
            );
        }
    }
}

/// The same thing, as words on his phone.
///
/// **Words, not a card.** A card per pair-and-timeframe is ten pictures and
/// ten seconds of Chrome each; this is one message he can read in a glance.
/// The cards are for a single thing that happened, not for a sweep.
pub async fn to_telegram(found: &[Found]) -> Result<()> {
    let client = reqwest::Client::new();
    let hits: usize = found.iter().map(|one| one.at_zones.len()).sum();

    let mut lines = Vec::new();

    lines.push(if hits == 0 {
        "🔍 <b>Nothing at your zones</b>".to_string()
    } else {
        format!("★ <b>{hits} at your zones</b>")
    });

    lines.push(String::new());

    // The answer first, because it is the question he asked.
    for one in found {
        for signal in &one.at_zones {
            lines.push(format!(
                "★ {}",
                reasons::sentence(signal, &one.symbol, one.interval.spoken(), one.digits)
            ));
        }
    }

    if hits > 0 {
        lines.push(String::new());
    }

    lines.push("<b>Every pair, both timeframes</b>".to_string());

    for one in found {
        let newest = one
            .shapes
            .last()
            .map(|(shape, _)| shape.as_str())
            .unwrap_or("—");

        lines.push(format!(
            "{} · {} — {} · last shape: {}",
            one.symbol,
            one.interval.spoken(),
            one.last.close.round_dp(one.digits),
            newest,
        ));
    }

    lines.push(String::new());
    lines.push(
        "<i>Shapes are what the code named, not an opinion. A shape away from a \
         zone is a description; only the ★ lines are at one.</i>"
            .to_string(),
    );

    telegram::send_words(&client, &OWNER.to_string(), &lines.join("\n"))
        .await
        .context("could not send the scan")?;

    println!("\nSent.");

    Ok(())
}
