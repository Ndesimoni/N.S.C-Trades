# Rung 3 — a shape, at a level

**Settled 25 August 2026.**

> **RECONSTRUCTED 29 AUGUST 2026.** The original of this file was deleted by
> mistake during the removal of the chart patterns, and it was never committed,
> so it could not be recovered. This version is rebuilt faithfully from
> `crates/nsc-strategy/src/README.txt` and `config/strategy.toml`, which
> together carried nearly all of it. **Anything the original said that is not
> in those two files is lost.** If something here reads wrong against your
> memory of it, your memory wins.

**This file is the specification.** The code is the guess; when they disagree,
this wins.

---

## The rule, in one sentence

> **A shape he trades, sitting at a level he drew.**

Three strategies were described on 25 August and they collapsed into one rule
with four kinds of shape, because **the place test turned out to be identical
for all of them.** Three rules that differ only in which shape they accept are
one rule with a list.

---

## Which shapes count

`nsc-ta::pattern` names eight. He trades four, each of which comes both ways up:

| shape | |
|---|---|
| `Push` | his own — a push, then a pin whose tail opposes it |
| `Engulfing` | a body that swallows the one before it |
| `Harami` | a big candle, then a small one hiding inside its body |
| `Marching` | three candles the same way, each closing beyond the last |

**Harami and marching were added on 29 August 2026, at his word.** This section
still said push and engulfing only until 31 August, which is two days of a
specification describing a bot that was not running.

The rest — tweezers, piercing, dark cloud and the star — exist because they are
on every candlestick page, not because they are on his chart.

**Left out rather than quietly included.** A detector that fires on eight
shapes when he trades four is twice the messages and half the meaning.

---

## Where the shape is measured from

**A push is measured from its pin's tail tip** — the low on an `nsc-bull`, the
high on an `nsc-bear`.

**Argued from what the pattern is, not chosen.** The tail is a pullback that
failed. If it reached into the level, the level is what stopped it, and that is
the whole story of the setup. Measuring from the body instead would pass a
shape whose rejection happened somewhere else entirely and whose body merely
ended up nearby.

**An engulfing has no tail to speak of**, so it is measured from where the
second candle closed — which is what "engulfing at the level" means to the eye.

**A harami is measured from its BIG FIRST candle**, never the small one. Price
travelled into the zone on the big candle; the small one is only the proof it
stopped there.

**A march is measured from where the run STARTED.** Three candles launch from
somewhere, and if a zone is there that is the zone it broke out of. Measuring
from the end would put the setup three candles away from the level that caused
it.

---

## The place test — half a band, and no touch rule

**Inside the band is inside.** There is no depth rule.

**Outside it, half of that band's own thickness.** `reach_of_band = "0.5"`.

**Never a distance.** A band on gold is about 78 points and on the euro about
0.004. Written as a price it would be right on one pair and quietly wrong on
every other one.

**And there is no touch rule at all.** Asked on 25 August whether the pin had
to touch the band, he said it need not, and that touching was no problem
either. So distance is the only test, and a pin that pokes inside measures as
nought.

---

## Only ever on a finished candle

Rung 2 has a "so far" card that reads a candle still running, and it says so on
its face. **A signal must never do that.** A shape halfway through a candle is
not a shape, and one that un-forms before the close would have been a message
about something that never happened.

`watch/closes/setups.rs` enforces it.

---

## It reports, it does not enter

**Version 1 sends signals and places no trades.**

Where the stop goes has not been settled, so a signal with no stop is a
*reading* rather than a trade. `reasons.rs` never writes buy, sell, entry,
target or stop, and a test pins that.

---

## Why the level is the whole point

`nsc-bull` and `nsc-bear` were measured across five pairs and five timeframes
on 22 August. Followed for ten candles they reached +1 normal candle before -1
in **29 of 75 — 38%**, where a coin flip is 50%.

**None of those had a level under them.**

So this rung is the test of the sentence `pattern/README.txt` already ends on:
a pattern is a description, and what makes one worth anything is the level it
printed at.

**If these come back at 38% as well, the level does not save it. That is a
finding, not a failure**, and it is worth more than another rule would be.

---

## It cannot reach anything

No `tokio`, no `reqwest`, no `sqlx` and no clock in `nsc-strategy/Cargo.toml`,
so nothing in the crate *can* fetch. The compiler refuses.

That is what lets the backtester and the live bot run these exact rules and
agree. **There is no "if we are backtesting" in there and there never may be** —
the moment there is, the backtest is testing something else, and the mismatch
makes results look *better* rather than broken.

---

## Still open

- [ ] **Where the stop goes.** Unanswered since 25 August. Until it is, every
      signal is a reading rather than a trade.
- [ ] **What makes him skip a rejection.** The other question left from
      25 August.
- [ ] **Nothing has measured whether the level helps.** The 38% above is
      without levels. The same measurement with them needs `nsc-backtest`,
      which does not exist.
- [ ] **Rejected setups are not saved.** That is a `CLAUDE.md` rule and this
      rung is what it was waiting on.
- [x] **The engulfing has a size test.** Done 29 August 2026: `min_reach =
      "1.0"` in `config/patterns.toml`, and `min_first_reach` for the harami's
      big candle. 39% of engulfings and 38% of haramis moved less than a normal
      candle before those went in, measured across 270,000 candles.
