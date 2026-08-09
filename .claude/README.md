# .claude/

Settings for Claude Code in this project.

| Path | What it is |
|------|-----------|
| `../CLAUDE.md` | **The rules.** Loaded automatically, every session. |
| `skills/`      | Playbooks for specific jobs. Loaded only when relevant. |

## Rules and skills are not the same thing

The difference decides where something belongs.

**`CLAUDE.md` is always loaded.** It holds the things that apply to every
change — never touch the outside world in the clean crates, never use future
data, measure in ATR, how errors work.

Keep it short. Everything in it costs space on every single request. If
something only matters during one kind of job, it is not a rule.

**Skills load only when they are relevant.** They hold how to do one specific
job — adding something to the chart reader, reading a settings sweep, writing
a database change. They can be longer, because you only pay for them when you
are doing that job.

The test: *does this apply to every change, or only one kind of change?*
Every change → rule. One kind → skill.

## The skills here

| Skill | Loads when |
|-------|-----------|
| `ta-primitive`   | Changing anything in `nsc-ta` |
| `strategy-rule`  | Changing your trading rules |
| `data-source`    | Adding or fixing a broker connection |
| `backtest`       | Running or reading a backtest |
| `db-migration`   | Changing the database |
| `workspace`      | Adding a crate or a dependency |
| `testing`        | Writing tests, especially golden files |
| `merge-check`    | Checking a change before it lands |
| `observability`  | Adding logging or health checks |
| `debug-live`     | Something is wrong with the running bot |
| `deploy`         | Putting it on a server |

There is deliberately no skill about git. The parts specific to this project —
never commit `.env`, bump the strategy version when rules change — are rules,
so they live in `CLAUDE.md`. The rest would be generic advice, and a skill that
just restates common practice costs attention without adding anything.

## Writing a new skill

`.claude/skills/<name>/SKILL.md`, starting with:

```markdown
---
name: my-skill
description: Use when <the specific situation>. Covers <what it does>.
---
```

The `description` is the only part read when deciding whether to load the
skill. Write it as a trigger, not a summary. "Use when adding a broker
connection, or when candle timestamps look wrong" beats "Information about
data sources".

Write down the reasoning, not just the steps. A checklist tells you what to
do. The reasoning tells you what to do when the situation is slightly
different from the one the checklist expected — which it usually is.
