# What you can send the bot

Everything here is typed into the **private chat** with your bot. Not the
channel — a channel post carries no sender at all, Telegram strips it, so the
bot cannot tell it is you.

The bot has to be running:

```sh
cargo run -p nsc-work-man
```

That one command is the watcher *and* the inbox. If it is not running, nothing
you send goes anywhere, and nothing tells you.

---

## `/level` — add a level

```
/level          →  it shows your pairs as buttons, plus "+ new pair"
tap GBPUSD      →  which timeframe?  [Weekly] [Daily] [4-hour]
tap Weekly      →  send prices
1.28 1.31       →  saved
```

**Send as many prices as you like**, one per line or all on one line. Nothing
asks how many — say four and send three and it would wait forever.

The pair and the timeframe stay put, so six weekly levels is two taps and six
numbers.

**A pair you have never sent starts its own file.** How many decimals it is
quoted to and whether it shuts at night are worked out from the name and
marked as unchecked — correct them if the pair behaves differently.

### What comes back

Two things. First what the pair **now holds**, all of it, not just what you
just sent:

> **GBP/USD · saved**
>
> **Weekly** — 3
> 1.14000 · 1.21279 · 1.28000

A mistyped `1.4000` is then caught by your eye in the reply, rather than three
weeks later when a signal fires in the wrong place.

Then a **picture** of where the bands landed. Reading a price back only proves
you can read your own typing. The picture shows the *place*, which is the thing
that actually goes wrong.

### Sending the same level twice

It saves once, and says so:

> **GBP/USD · 2 saved**, 1 you already had
> · 1.28000 is already a weekly level

**The price alone makes a level.** Sending 1.28 on the weekly and then 1.28 on
the daily is one line on your chart, so it stays one level — you would
otherwise get a 62-pip band and a 29-pip band around the same line, both firing
as price passed through.

It keeps the chart you first drew it on, and the reply says which.

---

## `↩ Undo` — take back what you just sent

Appears as a button after saving. It takes off **only what that last message
added** — so if two of the three you sent were already there, it takes off one.

It does not touch anything from an earlier message.

---

## `/remove` — stop watching a pair

```
/remove             →  which pair?
tap GBPUSD          →  "It has 4 levels on it. Stop watching it?"
tap ✓ Yes, stop it  →  stopped
```

**Two taps**, because it sets aside every level you have drawn for that pair,
and the first tap happens on a phone while you are doing something else.

**Nothing is deleted.** The file moves to `config/pairs/removed/`, and the
reply tells you exactly where it went.

Stop the same pair twice and both sets are kept — the second lands as
`GBPUSD-2`. You might add it back, draw it again, and drop it again, and the
first set is still the one you spent an evening on.

---

## `/restore` — put a stopped pair back

```
/restore        →  which one should I put back?
tap EURUSD      →  back, and being watched again
```

**One tap.** It takes nothing away, so there is nothing to be careful about.

It comes back under the pair's own name, whatever the file is called — restore
`GBPUSD-2` and it lands as `GBPUSD`.

**It refuses to land on a pair you are already watching.** If you stopped
GBP/USD, drew it again from scratch, and then restored the old set, it would
replace the levels you are using with the ones you put aside. It says so
instead:

> Could not put it back.
>
> GBP/USD is already being watched — stop it first

---

## What arrives without you asking

| | When |
|---|---|
| 👀 **approaching** | price comes near one of your zones |
| 🔔 **in the zone** | and again when it goes in |
| ⏳ **so far** | about a third of the way into a candle at your zone |
| 🕯 **closed** | that candle finishes — kissed it, pushed back, closed inside, cut through |
| 📐 **got it** | a level you just sent is now being watched |
| 🫀 **still running** | 07:00 UTC, and only on a day nothing else was sent |
| ⚠️ ✅ 🛑 | the price line went down, came back, or the bot stopped |

**Nothing arrives on a quiet hour.** That is the whole design — send something
every hour and by the second week you stop opening them, and then you miss the
one that mattered.

**Nothing at all arrives Saturday, Sunday or Monday.** The weekend is the
market being shut; Monday is your own rule. The heartbeat still goes out, or a
quiet Monday and a dead bot would look exactly the same.

---

## Seeing a card without waiting for it

Any message the bot can send, on demand:

```sh
cargo run -p nsc-work-man --bin cards -- XAUUSD              approaching
cargo run -p nsc-work-man --bin cards -- XAUUSD 4120         in the zone
cargo run -p nsc-work-man --bin cards -- XAUUSD 4120 found   already in
cargo run -p nsc-work-man --bin cards -- XAUUSD close        a candle's close
cargo run -p nsc-work-man --bin cards -- XAUUSD close 4375.6 sofar
cargo run -p nsc-work-man --bin cards -- heartbeat
cargo run -p nsc-work-man --bin cards -- armed
cargo run -p nsc-work-man --bin cards -- trouble down|back|stopped
```

Useful because some of these would otherwise take a week to see — a quiet-day
heartbeat, or the line going down and coming back.
