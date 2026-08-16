# 🔴 The bot answered nothing, and looked perfectly healthy

**Found** 16 August 2026, when he said `/status` had stopped working.
**Fixed** 16 August 2026 — `crates/nsc-work-man/src/card/chrome.rs`.

## What he saw

He sent `/status`. Nothing came back. `/help` — nothing. Every command dead.

The process was running. The log was clean. There was no error anywhere.

## Why

To stop Chrome clashing with his own open browser, Chrome had been given a
profile folder of its own — `--user-data-dir`.

With that flag, **Chrome writes the picture and then never exits.** The call
waiting on it waits for good, and because that call blocks, the whole bot
stopped answering.

There were twenty-five stuck Chrome processes on his machine.

Measured both ways: no flag, two seconds and it exits. With the flag, still
running after two and a half minutes.

**The clash it was meant to fix did not exist.** Modern headless Chrome makes
its own throwaway profile already. The real clash was two copies of the bot
writing the same card file — see `two-copies-eating-his-messages.md`.

## What it cost

Everything, silently. 🔴 for the third reason in the list: a bot that is up
and answering nothing is worse than one that is down, because he has no way to
tell.

## The fix

The flag is gone. And Chrome now gets a deadline it cannot outlive — one
minute, then it is killed. A card takes two seconds.

`card/waiting.rs`, three tests, both failure paths checked against the kill
being removed.

## The lesson worth keeping

**I tested that change and called it a pass, because the pictures appeared.**

They did appear. Chrome drew them and then sat there. Checking the file and
not the process is exactly how it got through — and the same reasoning nearly
missed it a second time.

When a change touches something that runs a program: **time it, and check the
process exits.**
