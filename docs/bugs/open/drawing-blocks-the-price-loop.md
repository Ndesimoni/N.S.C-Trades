# 🟠 Drawing a card blocks the price loop

**Where** `card/chrome.rs`, and the six places that draw a card
**Found** 16 Aug 2026

## What happens

Chrome is waited on with a blocking wait inside async code. For the 2–10
seconds a card takes, one worker thread does nothing else.

On his Mac: an alert can be a few seconds late. On a one-core box: everything
stops until the card is drawn.

## Fix

`tokio::task::spawn_blocking` at the six call sites. That pool needs owned
values, so each clones its inputs first.

Do it once a live session has run clean — it touches every path that sends.
