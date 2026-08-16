# 🟠 Drawing a card blocks the price loop

**Found** 16 August 2026, reading the code back.
**Where** `crates/nsc-work-man/src/card/chrome.rs` — `shoot`, and the six
places that call a card.

## What he sees

Today, on his Mac: almost nothing. An alert can arrive a few seconds late if a
card happens to be drawing at the moment price reaches a zone.

On a one-core box — which is where this is going — everything stops for the
two to ten seconds a card takes. No prices read, no `/status` answered, no
level picked up. Then it carries on as if nothing happened.

## Why

Chrome is run and waited on with a plain blocking wait, and that wait sits
inside async code. For as long as it lasts, one of Tokio's worker threads does
nothing but poll Chrome.

His Mac has eight or more of those threads, so prices keep arriving on the
others and queue in the socket buffer. One or two cores and there is nothing
left to arrive on.

## What it costs

Nothing is lost today — queued prices are read in order the moment the thread
comes back, and `arrive` sees them all.

It is 🟠 rather than 🟡 because the cost jumps the day it is hosted, and it
will not show up in any test on this machine. It is 🟠 rather than 🔴 because
no message is wrong, only late, and the deadline in `waiting.rs` caps the
worst case at a minute.

## The fix

`tokio::task::spawn_blocking`. Tokio keeps a separate pool for exactly this,
and work sent there never touches the threads running the price loop.

**The catch:** the card functions borrow — `&Pair`, `&Band` — and that pool
needs owned values. So each call site clones its inputs first. They are small
structs; the clone is nothing beside running Chrome.

**Six call sites:** the alert, the candle close, the heartbeat, `/status`, the
armed card, the trouble card.

Held back on 16 August because it touches every path that sends anything and
no live session had been watched yet. **Do it once one has run clean.**
