# nsc_trades

A trading signal bot. It reads charts and sends signals to Telegram.

**Version 1 sends signals and places no trades.**

---

# 👉 [OPEN THE DESIGN](https://claude.ai/code/artifact/1093ff9f-f3b3-4af7-afd5-6377629ea1dd)

**https://claude.ai/code/artifact/1093ff9f-f3b3-4af7-afd5-6377629ea1dd**

The whole thing on one page:

- the **two lanes** — the price watcher on every tick, the analysis on closed
  candles only — and the gate between them
- **what sits inside each stage**
- **where your rules live** — the six layers a setup has to survive
- **what lands on your phone**
- **the build order**, and what each step proves

---

## Where things are

| | |
|---|---|
| [`PROGRESS.md`](PROGRESS.md) | what is actually done, and what is next |
| [`docs/`](docs/README.md) | the design pages, and what the feed really sends |
| [`CLAUDE.md`](CLAUDE.md) | the rules this project is built to |
| [`assets/card/`](assets/card/) | what the bot sends — the design lives here, in HTML |
| [`crates/nsc-work-man/`](crates/nsc-work-man/src/README.txt) | the bot |
| `preview/` | the last card it drew. Open the `.html` in Chrome |

## Running it

```sh
cargo run -p nsc-work-man
```

Fetches gold's most recently **finished** 1-hour candle, draws a card, sends it
to your Telegram channel.

Needs a `.env` in the project root:

```
TWELVE_DATA_API_KEY=...
TELEGRAM_BOT_TOKEN=...
TELEGRAM_CHAT_ID=...
```

Never committed. `.gitignore` blocks it.

## What it does today

One pair, one timeframe, one message. That is step 1 of eight, and it is
finished — see [`PROGRESS.md`](PROGRESS.md).
