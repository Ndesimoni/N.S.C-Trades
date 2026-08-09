# research/  — the Python side (Phase 4, offline only)

**Nothing here runs in production and nothing here touches the live bot.**

It exists for one job Python does better than Rust: training the model that
scores your setups, and working out which of your confluences actually matter.

## The process

1. Export your judged signals — what the bot saw, what happened, what you
   said — from the database.
2. Train a tree-based model. Not a neural network: with a few hundred to a few
   thousand examples of plain numbers, trees win and are far easier to
   understand.
3. Ask the model which inputs it actually relied on. **This is the genuinely
   valuable output.** It tells you which of your confluences the data supports
   and which are decoration. Expect surprises, and expect to edit
   `config/strategy.toml` because of them.
4. Export the model so the bot can load it.

## Before you start

You need roughly 200–300 judged signals for any of this to be worth running.
Below that you will learn noise and believe it.

Phase 3 — running the bot and pressing 👍/👎 — is what produces them. There is
no shortcut.

## Do not

- Do not write anything back into the signals table.
- Do not recalculate what the bot saw. That gets worked out once, by the
  chart-reading code, and saved with the signal. Recalculating here would mean
  training on inputs the live bot never actually produces.
