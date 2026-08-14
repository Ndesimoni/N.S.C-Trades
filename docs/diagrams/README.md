# diagrams/

Pictures built to settle a question, or to check that two people mean the same
thing.

## The diagrams

### [The clock, not the stamp →](https://claude.ai/code/artifact/9088bc46-0abf-4d19-b33e-7f3ba4d2895a)

Every candle is labelled with the time it *opened*. A 4-hour candle running
21:00 to 01:00 is labelled 21:00, and so is every swing it confirms — but
nobody knew what it would do until 01:00.

**Settled** — the lookahead guard judges by the clock time a candle closed, not
the label it carries. Comparing labels fails both ways: 4-hour readings arrive
four hours early, and 15-minute swings that plainly happened get thrown out.
Found by reading the code back; two tests fail without the fix.

Source: [`clock-not-stamp.html`](clock-not-stamp.html)

### [How thick should a band be? →](https://claude.ai/code/artifact/bd6f98cf-a63a-458f-9cdd-a99430cb9b3a)

Six USDCAD levels drawn twice at different thicknesses. One pen width across
every chart looks consistent to the eye but cannot be computed; a share of a
normal candle can.

**Settled** — 0.35 of a weekly candle, 0.60 of a daily one, on every
instrument. Set as `drawn_weekly_atr` and `drawn_daily_atr` in `ta.toml`.

Source: [`band-thickness.html`](band-thickness.html)

### [Your levels, on the real candles →](https://claude.ai/code/artifact/a5a16539-c393-4783-ad87-5c87577046d7)

The eight gold levels he drew, put on the 15-minute export and built up into
weekly, daily, 4-hour, 1-hour and 15-minute. Same bands on every chart, because
a level is a price band and cannot change size with the zoom.

Source: [`gold-my-levels.html`](gold-my-levels.html)

### [Your lines against the code's →](https://claude.ai/code/artifact/c31231ba-328a-489f-9948-92ff02f34f88)

His six weekly levels beside the finder's, on the same six years.

**Settled, and it decided the whole approach** — the finder got four of his
eight, and three could never be found. His levels are where a big move *ended*;
the finder looks for prices where swings *cluster*. Different definitions, and
no setting bridges them. So the bot trades his levels.

Source: [`levels-vs-yours.html`](levels-vs-yours.html)

### [Real gold, weekly and daily levels →](https://claude.ai/code/artifact/99c24550-c8cd-42a1-a656-9bc93ee06d3c)

The finder running on six years of real gold. Ten levels found, nine drawn, one
hidden behind a weekly band.

Also holds the `max_age_bars` problem: 500 candles is two years on the daily
but ten on the weekly, so the daily search comes back nearly empty.

Source: [`gold-levels.html`](gold-levels.html)

### [Six levels, three lines →](https://claude.ai/code/artifact/58d7f1c9-6472-4092-bb99-578ae36eb925)

What happens when levels from three timeframes land on one chart. The bigger
timeframe keeps its line; the smaller one is marked, not deleted.

**Made-up prices**, and the page says so.

Source: [`level-absorbing.html`](level-absorbing.html)

### [Real gold, read two ways →](https://claude.ai/code/artifact/85930284-f032-4dca-ba5a-90d672d8ef4f)

The first page in this folder built on **real data** — 24,027 XAUUSD 30-minute
candles from Pepperstone, aggregated up to 4-hour and daily by the bot and read
by the real code.

Holds the proof that the aggregator is right: all 2,096 prices across 524 built
daily candles match the broker's own daily candles to the cent.

**Open** — whether the run floor measures against the biggest of recent runs or
the middle. The page shows both readings of the same data so it can be settled
by eye.

Source: [`gold-two-ways.html`](gold-two-ways.html)

### [The same rules at four timeframes →](https://claude.ai/code/artifact/9bb3407c-3896-4f6c-9608-7a20edd671f1)

One series of 30-minute candles, built up into 1-hour, 4-hour and daily through
the aggregator, then read by the same four settings. Every swing and level on
it came out of the real code.

Shows that more swings appear on lower timeframes because there are more legs
there, not because the rule loosened — the settings are identical on all four.

**Made-up candles**, and the page says so twice. The real version needs a gold
daily export.

Source: [`timeframes.html`](timeframes.html)

### [The candlestick shapes, and two still undecided →](https://claude.ai/code/artifact/9f9ef70d-c0d6-4bfb-b621-25a26f94339a)

The six shapes the bot now reads, drawn with the measurement that makes each
one. Three near misses it refuses, including the body-in-the-middle case that
slipped through until it was caught by a second read. And the two stubs —
inside bar and star — that are waiting on one answer.

**Open** — whether inside bars and stars get built or deleted. Written up in
[worksheets/candles.md](../worksheets/candles.md).

Source: [`candles.html`](candles.html)

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

### ~~What "3 candles either side" means~~ · [superseded →](https://claude.ai/code/artifact/cdaabf21-8e37-4597-b860-671f89426877)

**Describes a rule the project no longer uses.** Counting candles either side
of a peak was replaced by the run-and-pullback rule above.

Kept because it records *why*. Candle counting passed a lazy rounded top with
twenty quiet candles around it, and failed a sharp turn with four — neither
matching what you see on a chart. The page now opens with a banner saying so,
so it cannot be read as current.

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
