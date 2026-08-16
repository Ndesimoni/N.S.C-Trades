# 🔴 Two copies running, and neither said so

**Found** 16 August 2026.
**Fixed** 16 August 2026 — `crates/nsc-work-man/src/inbox/hearing.rs`.

## What he saw

Messages answered sometimes and not others, with no pattern. He would send
`/status` and get nothing, send it again and get a card.

## Why

Telegram hands each message to **whichever copy asks for it first**. Two bots
polling the same token split his messages between them at random, and each
one looks like it is ignoring him.

Telegram does say so. It refuses the second poller with error **409,
"Conflict: terminated by other getUpdates request"**.

Only the `result` field of the answer was ever read, and a refusal carries no
`result`. **So a refused poll looked exactly like a quiet minute** — and the
losing copy span on silently, forever, while he sent messages nothing was
reading.

## What it cost

🔴 for the same reason as the last one: it was up, it was quiet, and quiet is
supposed to mean nothing happened.

The same silence covered a **bad token** — which is a completely different
evening and read identically.

## The fix

The answer is checked, not just harvested. On 409 it names the problem every
fifteen seconds until one copy is shut down. Any other refusal is said too.

Four tests, including 409 and a rejected token.

## The lesson worth keeping

**A refusal that carries no data looks exactly like no data.** Anywhere a
reply is picked apart field by field, ask what the reply looks like when the
answer is no.
