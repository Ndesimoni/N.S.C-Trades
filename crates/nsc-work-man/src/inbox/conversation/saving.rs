//! A line of prices, saved, and said back to him.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{digits_for, save, with_slash};
use serde_json::json;

use super::super::picture::show;
use super::super::talking::say;
use super::super::{TIMEFRAMES, UNDO};
use super::adding::Adding;
use super::reading::prices_in;

/// Saves whatever numbers are in the message, if he is somewhere they can go.
pub async fn save_them(
    client: &reqwest::Client,
    token: &str,
    folder: &Path,
    text: &str,
    adding: &mut Adding,
) -> Result<()> {
    // Prices.
    let prices = prices_in(text);

    if !prices.is_empty() {
        let (Some(pair), Some(timeframe)) = (adding.pair.clone(), adding.timeframe) else {
            return say(
                client,
                token,
                "Send /level first, so I know what those are",
                None,
            )
            .await;
        };

        let saved = save(folder, &pair, timeframe, &prices, digits_for(&pair))?;

        // **Only what was actually added.** Undo cuts the last N levels off the
        // file, so telling it a number that includes ones already there would
        // have it cut levels he sent weeks ago.
        adding.just_added = Some((pair.clone(), saved.added));

        // Say back what the pair NOW HOLDS, not only what just arrived. A
        // mistyped 1.4000 is then caught by his eye in the reply rather than
        // three weeks later when a signal fires in the wrong place.
        let mut lines = vec![match (saved.added, saved.already.len()) {
            (_, 0) => format!("<b>{} · saved</b>", with_slash(&pair)),
            (0, _) => format!("<b>{} · nothing new</b>", with_slash(&pair)),
            (new, old) => format!(
                "<b>{} · {new} saved</b>, {old} you already had",
                with_slash(&pair)
            ),
        }];

        // **Name the ones he already had, and the chart they are on.** He may
        // have just re-sent a weekly line off his daily chart expecting it to
        // move; saying nothing would leave him thinking it had.
        for (price, timeframe) in &saved.already {
            lines.push(format!(
                "· {price} is already a <b>{}</b> level",
                timeframe.name()
            ));
        }

        for (word, kind) in TIMEFRAMES {
            let held: Vec<String> = saved
                .pair
                .levels
                .iter()
                .filter(|line| line.timeframe == kind)
                .map(|line| line.price.to_string())
                .collect();

            if !held.is_empty() {
                lines.push(format!(
                    "\n<b>{word}</b> — {}\n{}",
                    held.len(),
                    held.join(" · ")
                ));
            }
        }

        let names: Vec<&str> = TIMEFRAMES.iter().map(|(word, _)| *word).collect();
        let buttons = json!([names, [UNDO]]);

        say(client, token, &lines.join("\n"), Some(buttons)).await?;

        // Then show him where they landed.
        //
        // Reading the price back only proves he can read his own typing. The
        // picture shows the PLACE, which is the thing that actually goes wrong
        // — and it is how he reads a chart anyway.
        return show(client, token, &saved.pair).await;
    }

    say(client, token, "Send /level to add a level", None).await
}
