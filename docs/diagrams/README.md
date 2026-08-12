# diagrams/

Pictures built to settle a question, or to check that two people mean the same
thing.

## The diagrams

### [When a higher high is really a higher high →](https://claude.ai/code/artifact/45d262ab-5f17-49e3-bbde-9b7d2a5e96f5)

Taking out the old high is not enough. Price has to carry 40 to 50% of the
previous run past it, measured from the take-out — which refuses the poke that
looks like a breakout and turns straight back down.

Also holds the four calls made inside the code so they can be argued with: what
happens to a cross that stalls, why the yardstick changed from normal candles
to a share of the run, why how far it carried is kept, and what stands in for
the missing swing behind the first high of a chart.

**Open** — option 2, change of character, and whether an uptrend should insist
on higher lows. Written up in
[worksheets/structure.md](../worksheets/structure.md).

Source: [`higher-high.html`](higher-high.html)

### [A swing is proved by the pullback →](https://claude.ai/code/artifact/07ce73cb-6623-47be-a0df-e8d5fde64372)

The rule that replaces candle counting: a peak counts once price gives back
about half the run that made it. What it accepts, what it now refuses, why
swings end up alternating, and the floor the rule still needs so a flat market
does not fill with swings.

**Open** — how much of the run, and how small a run stops being a move.
Written up in [worksheets/swings.md](../worksheets/swings.md).

Source: [`swing-pullback.html`](swing-pullback.html)

### [What "3 candles either side" means →](https://claude.ai/code/artifact/cdaabf21-8e37-4597-b860-671f89426877)

What the `lookback` setting asks of a candle, the same chart read at 3 and at
5, and how long you wait before a swing can be used — half a day on the 4-hour,
three days on the daily.

**Open** — whether 3 matches the peaks you would point at, and whether major
and minor swings need separating.

Source: [`swing-lookback.html`](swing-lookback.html)

### [What repeated touches do to a level →](https://claude.ai/code/artifact/250dbca0-2122-4ff8-b720-8bb8a651f1ce)

Two beliefs about what repeated touches do to a level: the "worn out" view
against the "well defended" view, and the four settings that changed because of
it.

**Settled** — more touches makes a level stronger. Written up in
[worksheets/levels.md](../worksheets/levels.md).

Source: [`level-touches.html`](level-touches.html)

### [What makes a level strong →](https://claude.ai/code/artifact/2e60670f-5c45-485e-aab0-e5893ae4544b)

Eight things that can be measured about a level, each drawn weak on the left
and strong on the right. A menu to react to, not a set of rules.

**Open** — which of the six candidates are real, and what is missing.

Source: [`level-strength.html`](level-strength.html)

---

## How these are made

Plain HTML with inline SVG. No libraries, no build step, no internet.

They share one set of colours and typefaces so they read as a set. The blue is
the same blue you use for daily levels on your charts, on purpose. They work in
light and dark, because whoever opens the link might be in either.

**They are pages rather than pictures on purpose.** Text stays sharp at any
size, and when a rule changes the file gets edited. A saved PNG would go stale
the moment a setting changed, with no way to tell.

The source lives in the repo rather than only on the web for the same reason. A
diagram showing an old rule is worse than no diagram, because it gets believed.

## Why this one is `.md` and the ones in `crates/` are `.txt`

The `README.txt` rule is for folders sitting next to code, where the file is
read in a terminal as often as an editor.

`docs/` is markdown throughout, and markdown gives you links you can click.

## The rule for this folder

**A diagram is an argument, not evidence.**

Every one of these is drawn to make a point clearly. That is useful for
agreeing what a rule *is*. It proves nothing about whether the rule *makes
money*.

That question needs 200 judged signals and a backtest, not a picture.
