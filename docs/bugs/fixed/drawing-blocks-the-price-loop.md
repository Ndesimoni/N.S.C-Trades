# 🟠 Drawing a card blocks the price loop — fixed 30 Aug 2026

**Where** `card/chrome.rs`, and every place that draws a card
**Found** 16 Aug 2026 · **Fixed** 30 Aug 2026

## What happened

Chrome was waited on with a blocking wait inside async code. For the 2–10
seconds a card takes, one Tokio worker did nothing else.

On his Mac: an alert a few seconds late. On a one-core box: **everything stops
until the card is drawn** — no prices read, no messages answered. Hosting is
the plan, so that was the one that mattered.

## What it was NOT

He asked on 30 August whether this was because the project is not on axum. It
is not, and that is worth writing down so nobody rewrites the wrong thing:

- **The project is already async.** `tokio` is in `nsc-data` and
  `nsc-work-man`, and there are 86 async functions in the bot.
- **Axum is a web framework.** It serves HTTP. It has nothing to say about a
  subprocess wait.
- **The blocking was never network-shaped.** Chrome is a separate program; you
  start it and wait for the OS to say it exited. There is no future to poll,
  which is exactly what `spawn_blocking` exists for.

Axum would matter for one thing only: moving Telegram from `getUpdates` long
polling to a webhook, which needs a server and a public URL. That is a
deployment choice and it would not have changed this by a millisecond.

## The fix

`tokio::task::spawn_blocking` at all ten sites. The note said six; there were
ten.

```text
  watch/say.rs          the alert, and the close        the price loop itself
  watch/pulse.rs        the heartbeat
  watch/reload.rs       the armed card
  watch/trouble.rs      the trouble card
  inbox/asked.rs        /status
  inbox/coming.rs       /news
  review/picture.rs     /chart
  news/saying.rs        the calendar         already done
  closes/setups.rs      all three signal pictures, in ONE hop
```

That pool needs owned values, so each site clones its inputs first. They are
small structs and the clone is nothing beside running a browser.

**`card::Alive` had to stop borrowing.** It held a `&Pair`, which cannot cross
into the pool, so it owns its `Pair` now and the lifetime is gone from the
type. Three construction sites, one clone each, twice a day.

**The signal path draws all three of its pictures in ONE hop**, not three. Three
separate calls would hold three threads of the pool where one will do.

## What is still true

**Seven of the ten changed paths have not been run**, because they need TWS: the
alert, the close, the heartbeat, `/status`, `/chart` and the armed card. The
setup, trouble and news cards were drawn and sent, and the suite and clippy are
clean — but "compiles and is clean" is not "was watched working".

Worth re-reading the moment a live session runs.
