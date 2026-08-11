# diagrams/

Pictures built to settle a question, or to check that two people mean the same
thing.

## The diagrams

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
