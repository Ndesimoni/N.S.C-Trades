//! A chart he asked to see.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{Pair, Timeframe, load_pair, with_slash};

use super::super::talking::say;
use super::sending::{draw, sent};
use crate::places::PREVIEW;
use crate::review::Drawn;

/// Draws a pair on whichever chart he asked for, because he asked for it.
///
/// **Nothing was saved to get here**, so a failure is only a failure to draw.
/// It says so plainly rather than borrowing the wording from `landed`, which
/// would tell him his levels are safe when he never sent any.
pub async fn of_pair(
    client: &reqwest::Client,
    token: &str,
    folder: &Path,
    name: &str,
    chart: Timeframe,
) -> Result<()> {
    let Ok(pair) = load_pair(&folder.join(format!("{name}.toml"))) else {
        return say(client, token, "That pair's file will not read.", None).await;
    };

    let out = Path::new(PREVIEW).join("asked-for.png");

    match draw(&pair, chart, &out).await {
        Ok(drawn) => {
            let caption = caption(&pair, chart, &drawn);
            sent(client, &drawn, &caption).await
        }
        Err(trouble) => {
            println!("  -> Could not draw {name}: {trouble:#}");

            let words = format!(
                "Could not draw the {} chart for <b>{}</b> just now. Try again in a minute.",
                chart.name(),
                with_slash(name),
            );

            say(client, token, &words, None).await
        }
    }
}

/// What to say over the picture.
///
/// **It says when the chart is empty of levels.** 150 four-hour candles is
/// about twenty-five days, so a weekly level drawn two years ago is nowhere
/// near it. Without this the picture looks like the bands failed to draw.
pub(super) fn caption(pair: &Pair, chart: Timeframe, drawn: &Drawn) -> String {
    let head = format!("📈 <b>{}</b> — the {} chart.", pair.symbol, chart.name());

    match (drawn.on_it, drawn.altogether) {
        // **Everything he has is on it**, which is the ordinary answer on a
        // weekly. A pair with no levels at all lands here too and gets a plain
        // chart, rather than being told that none of its nought levels reached.
        (on_it, all) if on_it == all => head,

        (0, 1) => {
            format!("{head}\n\nYour level is outside what this chart covers. Try the weekly.")
        }

        (0, all) => {
            format!("{head}\n\nNone of your {all} levels reach this far in. Try the weekly.")
        }

        (on_it, all) => format!(
            "{head}\n\n{on_it} of your {all} levels {} on it. The rest are outside what it covers.",
            if on_it == 1 { "is" } else { "are" },
        ),
    }
}
