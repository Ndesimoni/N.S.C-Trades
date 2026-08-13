---
name: diagram
description: Use when drawing a picture to settle a question about how the bot reads a chart — comparing two readings of a rule, showing what a setting does, or putting real candles on screen with the swings and levels the code found. Also use when a diagram in docs/diagrams/ needs updating because the rule it shows has changed.
---

# Drawing a picture to settle a question

Diagrams here have one job: **make two people agree what a rule is.**

Not to teach, not to decorate, not to summarise. If a paragraph would do it,
write the paragraph.

## Before drawing anything

Ask what the picture is arguing. One sentence. If you cannot say it, there is
no diagram to make yet — go back and find the disagreement first.

The good ones so far all did one of three things:

- **Two readings of the same data**, side by side, so the trader can point at
  the one that matches what he sees
- **What a setting does** — the same chart at two values of one number
- **Real candles**, with the swings and levels the actual code found on them

## Where it goes

```
docs/diagrams/
  README.md          the index — links at the top
  <name>.html        one self-contained page
```

Update `README.md` **in the same change**. It carries the published link, one
line on what the diagram argues, and whether the question is settled or open.

A diagram nobody can find is a diagram nobody reads.

## Say what the candles are

**Every page must say whether the candles are real or made up**, near the top,
where it cannot be missed.

Made-up candles are fine for showing what a rule does. They are worthless as
evidence that the rule is any good, because they were drawn to make the point.

Real data pages should say the instrument, the broker and the count —
"24,027 XAUUSD 30-minute candles from Pepperstone" — so the reader knows
exactly what they are looking at.

## Colours that are not yours to choose

The level colours come from the trader, in `docs/worksheets/levels.md`:

| Colour | Timeframe |
|---|---|
| Black | Weekly |
| Blue | Daily |
| Yellow | 4-hour |

**Never draw levels in one colour when more than one timeframe is on screen.**
That mistake was made once already and the chart looked nothing like his.

Everything else — the page palette, the accent — is a design choice. These
three are a specification.

## House style

One self-contained HTML file. Inline SVG. No libraries, no build step, no
internet, no external fonts.

Both light and dark, always. Tokens on bare `:root` for light, redefined under
`@media (prefers-color-scheme: dark)` guarded with
`:root:not([data-theme="light"])`, and again under `:root[data-theme="dark"]`.
Never put a colour only inside a media block — the page then renders one
theme's text on the other theme's background.

Georgia for headings, system sans for body, system mono for labels and
settings. They share one palette across every page in the folder so the set
reads as one voice.

Patterns that have worked:

- **Weak on the left, strong on the right**, with a divider between
- **`settled` / `open` chips** in the corner of each card
- **A ladder** at the top when the page is one rung of a bigger question

## Keep it honest

**A diagram is an argument, not evidence.** Every one of these is drawn to
make a point clearly. That is useful for agreeing what a rule *is*. It proves
nothing about whether the rule *makes money* — that needs judged signals and a
backtest.

Say so on any page that could be mistaken for a result.

## When the rule changes

**A diagram showing an old rule is worse than no diagram, because it gets
believed.**

When you change a rule, go and look at the pictures of it. Either update the
page and republish to the same URL, or mark it superseded in `README.md` and
say what replaced it.

`swing-lookback.html` still describes candle counting, which the run-and-
pullback rule replaced. That is the failure this section exists to stop.

## Publishing

Publish with the Artifact tool, then **copy the source into `docs/diagrams/`**
and put the link in `README.md`.

The source lives in the repo so the picture can be changed when the rule
changes. A page that only exists on the web cannot be edited by whoever finds
the mistake.
