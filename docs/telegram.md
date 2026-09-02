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

**You do not have to remember any of this.** The bot registers its commands
with Telegram, so they appear in a tap-list next to the message box.

---

## `/status` — is it running, and is anything close?

One card, whenever you like:

> 🫀 **Still running** · 4 pairs · 16 zones
>
> XAU/USD ● ● ●  **281.70** from weekly 4,094.00
> GBP/USD ● ● ● ●  **0.01724** from daily 1.37053

The last column is the useful part — **the nearest zone on every pair.** It
tells you what is brewing today without opening a chart.

It is the same card as the morning heartbeat. That one only comes on a day
nothing else did; this is the same question asked whenever you want.

**On a day nothing is watched it says so in words instead:**

> 😴 **Resting**
>
> The market is shut, or today is one you have set aside. Nothing is watched
> and nothing is fetched.
>
> **4** pairs · **13** zones are loaded and ready.

The card's useful column is how far price is from the nearest zone — and on a
quiet day no price has arrived, so every row would be a dash.

**It always answers.** If the picture cannot be drawn it sends the words on
their own. Replying "could not do that" to *is it running* is the one answer
that would be worse than none.

**On a day nothing is watched it says so in words instead:**

> 😴 **Resting**
>
> The market is shut, or today is one you have set aside. Nothing is watched
> and nothing is fetched.
>
> **4** pairs · **13** zones are loaded and ready.

The card's useful column is how far price is from the nearest zone — and on a
quiet day no price has arrived, so every row would be a dash.

**It always answers.** If the picture cannot be drawn it sends the words on
their own. Replying "could not do that" to *is it running* is the one answer
that would be worse than none.

---

## `/news` — what is coming up

Tap it and you get two buttons.

| | |
|---|---|
| **📅 Today** | The whole day |
| **🗓 This week** | The whole week, grouped by day |

**Both show everything, and every row says which side of now it is on.** One
that has gone is marked `PASSED` and greyed. One still to come carries how
long — *in 45m*, *in 10h 53m*, *in 3d 10h*.

Nothing is left out. A week with its first three days missing does not read as
a week, it reads as a quiet one — and "nothing left today" and "three already
gone" are different afternoons.

The heading counts both: **1 gone · 17 to come**.

Each row is the time, the currency, what it is, and a coloured stripe for how
hard it usually hits. **Red is high, orange is medium** — the same colours
ForexFactory uses, so you never have to translate.

**Low impact is left out.** It is three quarters of the file and it moves
nothing. What counts is set in `config/news.toml` and it is the same setting
the automatic warnings use, so the list and the cards can never disagree.

On a day with nothing on it you get a line of text rather than a card. Running
Chrome for ten seconds to draw the word "nothing" is not worth it.

---

## `/help` — the list

Every command with one line on what it does. Also `/start`.

---

## `✗ Close` — back out

**On every set of buttons, on its own row.** Tap it and the buttons go away,
your own keyboard comes back, and the bot forgets what it was in the middle of.

There was no way out before. Once the buttons were up they stayed up, over your
own keyboard, until you finished what you started — and a half-finished
`/remove` looked exactly like a bot waiting for something.

**It takes nothing away.** Closing part-way through `/level` does not undo the
levels you already saved in that conversation; it just stops asking.

---

## `/pairs` — see everything, and change one

```
/pairs          →  every pair you have
tap GBPUSD      →  what it holds, and what you can do
```

> **GBP/USD** — 4 levels
>
> **Weekly** — 1.14000 · 1.21279 · 1.28000
> **Daily** — 1.37053
>
> `[+ Add levels]` `[− Take one off]` `[✗ Stop watching]`

**`− Take one off`** is the one that was missing. It shows each level as its own
button; tap one and it goes.

Undo only ever reached what the *last message* added. That covers a typo the
moment it happens — it does nothing for *"that 1.15 from last week was wrong"*.

The price is matched **as a number**, so tapping `1.15000` works whether you
typed 1.15 or 1.15000. And the comments in the file survive it, because you are
meant to be able to open one and read it.

There is no "change a price". Take the old one off and send the new one — two
taps and a number, and nothing can half-happen.

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
| 🕯 **broke** | a 4-hour or daily candle **breaks through** the level, the way price was travelling |
| 🎯 **setup** | a shape you trade prints at one of your zones — 1-hour, 4-hour or daily |
| ✅❌ **did you take it?** | straight after a setup, two buttons: *took it* and *skipped it* |
| 📅 **coming up** | 5 minutes before a high or medium impact release, and again 1 minute before |
| 📐 **got it** | a level you just sent is now being watched |
| 🫀 **still running** | 07:00 UTC, and only on a day nothing else was sent |
| ⚠️ ✅ 🛑 | the price line went down, came back, or the bot stopped |

## Two messages, and nothing else

Settled 1 September 2026. Your words: *"when price is getting to a level we do
not want an alert, so remove the card. We should only get alerts if the price
came from below the band level and closed above it, and vice versa. As for the
setups, candlestick patterns, it stays the same."*

```text
    price comes up to the level         silence
    it enters the zone                  silence
    a candle closes INSIDE it           silence
    it is thrown back out               silence   <- but see setups
    a candle opens below, closes above  🕯 broke   <- this is the message
```

**A break is read off the candle, not off where price is now.** Came from below
and closed above. Came from above and closed below. Anything else is not a
break.

**A rejection reaches you as a setup instead**, and that is the better half of
the deal. A rejection at a level *is* the reversal you trade — and the setup
card names the shape that made it, where a close card could only say the candle
ended below.

**Each timeframe keeps its own story.** A 4-hour candle breaking your weekly
level and a daily candle breaking it are two different pieces of news about one
line, and the daily is the bigger one.

**The 1-hour sends setups and nothing else.** Your call, 31 August 2026. It is
still watched, still fetched and still judged — a candlestick pattern at a zone
is the whole reason it is there.

**Two lines to flip if it goes too quiet**, both in `config/levels.toml`:

| | |
|---|---|
| `only_breaks = false` | every candle that touched a zone reports what it did there |
| `[close_cards] h1 = true` | the 1-hour narrates its closes as well |

There is no setting that brings back the approaching card. It was taken out
rather than turned off, along with the machinery that made it fire once per
visit — it is in git at `99ed9f1` if you want it again.

**Nothing arrives on a quiet hour.** That is the whole design — send something
every hour and by the second week you stop opening them, and then you miss the
one that mattered.

**Nothing at all arrives Saturday, Sunday or Monday.** The weekend is the
market being shut; Monday is your own rule. The heartbeat still goes out, or a
quiet Monday and a dead bot would look exactly the same.

**News warnings are the exception and they keep their own clock.** They are not
about your levels, so they do not wait on the price line or on the session —
Tokyo CPI prints on a Sunday evening whether or not anything is being watched.

---

## Seeing a card without waiting for it

Any message the bot can send, on demand — **and two it no longer sends.** The
first two draw the approaching card, which went on 1 September 2026; they are
kept because the card art is still worth looking at and the wording is what
approach alerts would use if they ever came back.

```sh
cargo run -p nsc-work-man --bin cards -- XAUUSD              approaching (retired)
cargo run -p nsc-work-man --bin cards -- XAUUSD 4120         in the zone (retired)
cargo run -p nsc-work-man --bin cards -- XAUUSD 4120 found   already in
cargo run -p nsc-work-man --bin cards -- XAUUSD close        a candle's close
cargo run -p nsc-work-man --bin cards -- news                what is coming up
cargo run -p nsc-work-man --bin cards -- news busy           several at once
cargo run -p nsc-work-man --bin cards -- news today          the day's list
cargo run -p nsc-work-man --bin cards -- news week           the week's list
cargo run -p nsc-work-man --bin cards -- heartbeat
cargo run -p nsc-work-man --bin cards -- armed
cargo run -p nsc-work-man --bin cards -- trouble down|back|stopped
```

Useful because some of these would otherwise take a week to see — a quiet-day
heartbeat, or the line going down and coming back.

---

## The two buttons under a setup

They arrive as their own short message right under the three pictures, because
**Telegram does not allow buttons on a group of photos.** The line names the
setup, so a tap can never land on the wrong one.

```
    ✅ took it        ❌ skipped it
```

Tap either. Tapping again changes nothing; tapping the other one changes your
mind, and the record keeps when you settled rather than when you first
wavered.

**Then say why, in your own words:**

```
    /why the wick ran into news, I stood aside
```

It attaches to the setup that arrived most recently — the one you almost
always mean. It needs a verdict first, so tap before you explain.

**There is no third button.** *"Would have skipped"* is what you say later,
once the outcome came in — a button for it would invite you to answer before
the market had.

**Why it matters more than it looks.** Candles can be downloaded again and
outcomes recomputed from them forever. What you thought of a setup on the
afternoon it printed exists nowhere else the moment you forget it.
