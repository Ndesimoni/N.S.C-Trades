//! Reading `.env`, and saying so when it will not read.
//!
//! **This exists because `dotenvy::dotenv().ok()` fails in silence**, and it
//! fails in the worst possible shape.
//!
//! `dotenvy` stops at the **first** line it cannot parse, and everything after
//! that line is simply not loaded. So one bad line halfway down takes out
//! every setting below it, `.ok()` throws the reason away, and what reaches
//! you is *"TELEGRAM_BOT_TOKEN is not set"* about a line plainly sitting there
//! in the file.
//!
//! That happened on 20 August 2026. The line was:
//!
//! ```text
//!     IBAPI_TIMEZONE_ALIASES=Gulf Standard Time=Asia/Dubai
//! ```
//!
//! An unquoted value with spaces in it. `dotenvy` refuses it — its own parser
//! comment says so, *"throwing on: k=v w"* — and both Telegram settings were
//! below it, so the bot could reach IBKR and could not say a word.

/// Where the settings live.
const FILE: &str = ".env";

/// Read `.env`, and say plainly if it could not be read.
///
/// **Not fatal.** On a server the settings may come from the real environment
/// with no `.env` at all. What must not happen is silence — the missing
/// setting is then reported by whoever needed it, and that reads as the truth
/// rather than a puzzle.
pub fn load() {
    let Err(trouble) = dotenvy::dotenv() else {
        return;
    };

    // No `.env` at all is perfectly normal. The settings are expected to be in
    // the environment instead.
    if trouble.not_found() {
        return;
    }

    eprintln!("\n⚠️  {FILE} could not be read fully, so some settings did not load.");

    let dotenvy::Error::LineParse(rest, _) = &trouble else {
        eprintln!("    {trouble}\n");
        return;
    };

    // **`rest` is the VALUE, not the line**, whatever the variant is called —
    // `dotenvy` has already eaten the name by the time it gives up. So it can
    // be a bot token, and it is never printed. The file is read again to turn
    // it into a line number and a name, which are safe to say out loud.
    match std::fs::read_to_string(FILE)
        .ok()
        .and_then(|text| found_in(&text, rest))
    {
        Some((line, name)) => {
            eprintln!("    Line {line} sets {name}, and could not be read.");
            eprintln!("    NOTHING BELOW LINE {line} WAS LOADED.");
        }
        None => eprintln!("    One line could not be read, and nothing below it was loaded."),
    }

    eprintln!("    A value with spaces in it has to be quoted:");
    eprintln!("        SOME_KEY=\"a value with spaces\"\n");
}

/// Which line ends with `rest`, and what that line is called.
///
/// **Only the name comes back, never the value.** Names are things like
/// `TELEGRAM_BOT_TOKEN`; values are the secret half, and this whole function
/// exists so that the secret half stays in memory.
fn found_in(text: &str, rest: &str) -> Option<(usize, String)> {
    let rest = rest.trim_end();

    text.lines()
        .enumerate()
        .find(|(_, line)| line.contains('=') && line.trim_end().ends_with(rest))
        .and_then(|(at, line)| {
            let (name, _) = line.split_once('=')?;

            Some((at + 1, name.trim().to_string()))
        })
}

#[cfg(test)]
mod tests {
    use super::found_in;

    const FILE: &str = "\
# a comment
FIRST=fine
IBAPI_TIMEZONE_ALIASES=Gulf Standard Time=Asia/Dubai
TELEGRAM_BOT_TOKEN=1234:secret
";

    /// It finds the line by its value and reports the line's NAME.
    #[test]
    fn it_names_the_line_that_stopped_it() {
        let found = found_in(FILE, "Gulf Standard Time=Asia/Dubai");

        assert_eq!(found, Some((3, "IBAPI_TIMEZONE_ALIASES".to_string())));
    }

    /// **The value never comes back**, because the value is the secret half.
    ///
    /// A bot token sitting in a terminal ends up in every log and screenshot
    /// of what went wrong — and the message that says what broke is exactly
    /// the one that gets pasted to somebody.
    #[test]
    fn it_never_hands_back_the_value() {
        let (_, name) = found_in(FILE, "1234:secret").expect("it should find it");

        assert_eq!(name, "TELEGRAM_BOT_TOKEN");
        assert!(!name.contains("1234"), "the value came back in the name");
    }

    /// A value it cannot place says nothing rather than guessing at a line.
    #[test]
    fn a_value_it_cannot_place_finds_nothing() {
        assert_eq!(found_in(FILE, "not in this file"), None);
    }
}
