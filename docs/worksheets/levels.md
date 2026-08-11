# Worksheet — Levels

How you draw support and resistance, in your own words.

Different from the other worksheets in this folder. Those describe **setups**
and become `config/strategies/*.toml`. This one describes **what you see on a
chart**, and becomes `nsc-ta` — the chart-reading code, which has no opinion
about whether to trade.

Captured 11 Aug 2026 from four screenshots: XAUUSD on 4h and daily, GBPUSD on
weekly and daily.

---

## What you told me

### Levels belong to a timeframe, and keep that tag everywhere

You colour them by where they came from:

| Colour | Timeframe |
|---|---|
| Black | Weekly |
| Blue | Daily |
| Yellow | 4-hour |

The important thing from the screenshots: **the same levels appear on every
chart.** The gold daily levels are drawn on the 4-hour. The GBPUSD weekly
levels are drawn on the daily.

So there is not "a set of levels for the 4h chart". There is **one set of
levels**, each tagged with the timeframe it was found on, and every chart
shows all of them.

Some charts were missing a colour simply because you had not drawn those in
yet — not because they do not apply there.

### Weekly and daily matter most

The 4-hour levels are real, but they act as **confirmation**. Weekly and daily
are the ones that decide things.

### How thick the band is

You set the thickness to catch the most touches. Some candles push through it,
some only touch the tip.

**The thickness is uniform** — about the same for every level on a given
timeframe. You slide the band up and down to catch the most touches; you do
not stretch it.

Measured off your screenshots:

- GBPUSD weekly bands: roughly 0.008 to 0.010 wide
- Gold 4-hour bands: roughly 20 points wide

Both come out at **roughly half a normal candle** on their own timeframe. That
is what the code will use, and it is a setting in `config/ta.toml` so it can
be tuned later.

### How far left you look

You look at what price has done before, but not too far back.

From the screenshots:

- Weekly: about 3 years
- Daily: about 2 years
- 4-hour: about 3 months

All three come out near 500 candles, which is what `max_age_bars` in `ta.toml`
is already set to.

### More touches makes a level STRONGER

Settled 11 Aug 2026.

A level that has been touched many times is one you pay **more** attention to,
not less. Lots of touches means lots of people are watching that price and
defending it.

This is worth flagging because the config originally assumed the opposite.
There is a well-known idea that a level tested repeatedly is being "worn down"
and will eventually break, so you should avoid it. That is not how you trade,
so the settings were wrong and have been changed:

| Was | Now |
|---|---|
| `exhaustion_touches` — worn out after 4 | `strong_touches` — strong at 4 or more |
| `skip_exhausted_levels = true` | `require_strong_level` |
| `level_untested` scores points | `level_well_defended` scores points |

**One thing to keep separate.** A well-defended level that finally *breaks* is
a big event, and probably a bigger one than a weak level breaking. That is a
breakout rule, not a level-strength rule, and it does not contradict any of
the above.

**Worth measuring later.** Once there are 200 labelled signals, this belief is
testable rather than assumed. It is written down here so it can be checked.

### Support that becomes resistance is the same level

If a support turns into resistance, it stays on the chart as one level. It
does not get redrawn or renamed.

**What that means for the code:** there is one `Level` type. Not a `Support`
and a `Resistance`. Which side price is on is just where price happens to be
today.

### Not every level gets drawn

You leave some out to keep the chart readable. They still matter.

**What that means for the code:** find them all and score them, then show only
the strongest few in the Telegram message. You get a clean chart without
throwing the information away — and in six months you can ask whether the ones
you skipped were worth taking.

---

## The trading rules that came out of this

These are **not** level-drawing rules. They belong in `nsc-strategy`, and they
are written here so they are not lost.

### Weekly levels are a hard stop

- Price sitting at a weekly level → **do not trade.**
- It breaks the weekly level → **still do not trade.**
- It breaks, retests, and wants to continue → **now you are clear to look for
  entries.**

This is a veto, and a strong one. Most traders never write this down.

### Daily levels are looser

You do not always wait for the retest. A powerful bullish or bearish candle
closing beyond the level can be enough to go looking for an entry.

### There are exceptions

Sometimes you would take the breakout on the weekly or daily. Those exceptions
have not been described yet, and they are where the real rule lives.

**To collect:** next time you take one, screenshot it and say why. The
difference between the breakout you took and the ones you skipped is the rule
that is currently missing.

---

## Still open

1. **How many touches before a band counts as a level at all?** `ta.toml` says
   2. Your bands look like they have four or more.
2. **When is a level worn out?** `ta.toml` says 4 touches. Do you actually
   stop trusting a level after that many, or does it get stronger?
3. **What makes a level worth drawing** versus one you leave off the chart?
4. **The exceptions** to the weekly rule.

---

## What this becomes

`nsc-ta::levels`, driven by `[levels]` and `[proximity]` in `config/ta.toml`.

None of the trading rules above go in there. `nsc-ta` reports what it sees;
`nsc-strategy` decides what to do about it.
