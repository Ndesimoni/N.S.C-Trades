//! Pairs IBKR will never serve, taken out of the way at startup.

use std::path::Path;

use nsc_core::levels::{known, load_pair, retire};
use nsc_data::sources::ibkr::{IbkrConnection, Serves};

use crate::places::{OWNER, PAIRS};
use crate::telegram;

/// Retire every pair IBKR has never heard of. **Once, at startup.**
///
/// **Moved, never deleted.** It goes to `config/pairs/removed/` and comes back
/// with one `/restore` — the same door `/remove` uses. A pair file is months
/// of chart work, and the cost of being wrong about one has to stay one tap.
///
/// **A pair it cannot ASK about is left completely alone.** TWS being shut is
/// an ordinary Tuesday, and if "could not ask" ever read as "no such pair",
/// one gateway outage would retire every pair he owns in a single startup.
/// That is why `serves` gives three answers and not a `bool`.
///
/// **He is told on Telegram, not in the terminal.** A pair vanishing from the
/// watch list is exactly the kind of quiet change that gets noticed weeks
/// later, when nothing has fired and he cannot tell whether that is the market
/// or the bot.
pub(super) async fn the_unservable(client: &reqwest::Client, ibkr: &IbkrConnection) {
    let mut gone = Vec::new();

    for name in known(Path::new(PAIRS)) {
        // **The symbol is READ, never guessed from the file name.** They are
        // not the same thing: `/restore` puts a pair back as `GBPUSD-2.toml`
        // when he has retired one twice, and working `GBPUSD-2` up into a
        // symbol gives nonsense — which IBKR would rightly say it had never
        // heard of, and this would retire a pair he is actually using.
        //
        // A file that will not parse is left alone. He may be halfway through
        // editing it by hand, and an unreadable file is not an unknown pair.
        let Ok(pair) = load_pair(&Path::new(PAIRS).join(format!("{name}.toml"))) else {
            continue;
        };

        // Only a definite "never heard of it" counts. Everything else — a
        // dropped line, a gateway starting up — leaves the pair exactly where
        // it is, to be asked about again next start.
        let Ok(Serves::Never { why }) = ibkr.serves(&pair.symbol).await else {
            continue;
        };

        match retire(Path::new(PAIRS), &name) {
            Ok(landed) => {
                eprintln!(
                    "{name} — IBKR will not serve it. Moved to {}",
                    landed.display()
                );
                gone.push((name, why));
            }

            // It could not be moved. Say so and leave it: the alternative is
            // pretending it was dealt with.
            Err(trouble) => {
                eprintln!("{name} — IBKR will not serve it, and it would not move: {trouble}")
            }
        }
    }

    if gone.is_empty() {
        return;
    }

    say_so(client, &gone).await;
}

/// Tell him which pairs went, and how to get one back.
async fn say_so(client: &reqwest::Client, gone: &[(String, String)]) {
    let list = gone
        .iter()
        .map(|(name, why)| format!("• <b>{name}</b> — <i>{why}</i>"))
        .collect::<Vec<_>>()
        .join("\n");

    let words = format!(
        "⚠️ <b>Stopped watching {} pair{}.</b>\n\n{list}\n\n\
         IBKR has no instrument by {} name, so nothing could ever have been \
         reported. The levels are <b>not lost</b> — send <b>/restore</b> to put \
         one back, or add it again under the right spelling.",
        gone.len(),
        if gone.len() == 1 { "" } else { "s" },
        if gone.len() == 1 { "that" } else { "those" },
    );

    // **A message that will not send must not stop the bot starting.** The
    // pairs are already out of the way either way, and the terminal has said
    // so. Refusing to start over a Telegram hiccup would be a worse outcome
    // than a message he reads later.
    if let Err(trouble) = telegram::send_words(client, &OWNER.to_string(), &words).await {
        eprintln!("Could not say which pairs were retired: {trouble:#}");
    }
}
