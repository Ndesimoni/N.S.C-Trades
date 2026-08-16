# Bugs

Everything found and not yet fixed. One file per bug.

```
open/     still there
fixed/    dealt with, kept for the reason it happened
```

**Nothing is deleted.** A fixed bug moves to `fixed/` with what fixed it,
because the reason it happened is worth more than the fix — three of the fixes
in this project came from reading an old one back.

---

## How bad is it

Every bug carries one of these on its first line. **The mark is about what it
costs him, not how hard it is to fix.**

| | | |
|---|---|---|
| 🔴 | **SERIOUS** | deal with before anything else |
| 🟠 | **REAL** | a message is lost or wrong; fix it soon |
| 🟡 | **SMALL** | wasted work or wrong wording; nothing is lost |

### What counts as 🔴

Only these four. Nothing else earns it, so the mark keeps meaning something.

- **A wrong signal**, or an alert at a place price never reached. He could
  trade on it.
- **Reading price the market had not printed yet.** It does not error. It
  makes a backtest look *better*, which is what makes it dangerous.
- **A dead bot that looks alive**, or a quiet day that looks like a dead bot.
  He stops trusting silence, and silence is the whole design.
  **This includes a message that should have come and did not.** Nothing
  arriving means nothing happened — so a missing message is not a smaller
  version of a wrong one, it says something false just as loudly.
- **A secret in a message** — the bot token or the API key.

Everything else is 🟠 or 🟡, however annoying.

---

## What a bug file looks like

Named for what goes wrong, not where: `stale-button-wrong-pair.md`, not
`one-rs-bug.md`. The file it lives in changes; what it does to him does not.

```markdown
# 🟠 A level button from an older message hits the wrong pair

**Found** 16 August 2026, reading the code back.
**Where** crates/nsc-work-man/src/inbox/dropping.rs

## What he sees

Walk it through with real numbers. What he taps, what arrives, what is wrong.

## Why

The cause, in one or two sentences.

## What it costs

Who is hurt and how badly. This is what the mark above is chosen from.

## The fix

What to do. Not code unless the code is the shortest way to say it.
```

---

## Open

Nothing outstanding at the last sweep — see the list below.

| | Bug | Found |
|---|---|---|
| 🟠 | [Drawing a card blocks the price loop](open/drawing-blocks-the-price-loop.md) | 16 Aug |
| 🟡 | [It only runs from the project root](open/only-runs-from-the-project-root.md) | 14 Aug |

**Last swept:** 16 August 2026.

---

## Fixed

Kept for the reason, not the fix.

| | Bug | Fixed |
|---|---|---|
| 🔴 | [The report of where price stands never came](fixed/greeting-reported-nothing-and-marked-itself-done.md) | 16 Aug |

**This folder starts with that one**, on 16 August 2026, and it is the last
bug found before the folder existed.

Everything before it stays in the git log. Writing old ones up from memory
would mean guessing at detail — and a bug file that guesses is worse than no
bug file, because it gets believed. **From here on, one goes in as it is
found.**
