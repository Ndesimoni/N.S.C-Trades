---
name: workspace
description: Use when adding a crate, adding or upgrading a dependency, or deciding which crate some new code belongs in.
---

# Crates and dependencies

## Which crate does this go in?

Two questions, in order:

**1. Does it touch the outside world?** If yes, it cannot go in `nsc-ta` or
`nsc-strategy`. Those two never touch a database, the internet, or the clock,
and that is what makes backtest results mean anything.

**2. Does more than one crate need it?** If yes, it belongs in `nsc-core`.
Never write a second `Candle` or `Level` somewhere else. If two crates have
their own idea of what a candle is, backtest and live quietly stop agreeing.

```
nsc-core                    types, almost no dependencies
   ↑
nsc-ta                      reads the chart, no outside world
   ↑
nsc-strategy                your rules, no outside world
   ↑
nsc-backtest   nsc-live     the two things that drive it
                  ↑
      nsc-data nsc-risk nsc-news nsc-ai nsc-chart nsc-telegram
```

The clean crates never reach down into the messy ones. Ever.

## Adding a crate

1. `crates/<name>/Cargo.toml`, inheriting version and edition from the root
2. Add it to the root `[workspace.dependencies]` as a path
3. A comment at the top of its `Cargo.toml` saying what it owns
4. `//!` docs in `lib.rs` — what it does, and where it matters, why it works
   this way rather than the obvious way
5. An `error.rs` — typed errors for a library, `anyhow` for a program
6. Update the tables in `README.md` and `.claude/README.md`

## Adding a dependency

Put the version in the root `[workspace.dependencies]` once, then refer to it
from each crate. Two versions of the same library in one project means slow
builds and confusing errors.

Before adding anything to `nsc-ta` or `nsc-strategy`, stop. A clean crate
needing a new dependency usually means the code belongs somewhere else.

## Version notes

- `sqlx` stays on 0.8 — 0.9 needs a newer Rust than this project uses
- `reqwest` stays on 0.12 — 0.13 renamed its TLS options
- Run `cargo generate-lockfile` before `cargo check`. It works out versions in
  seconds instead of failing several minutes into a build

## Before committing

```sh
cargo fmt --all
cargo clippy --workspace -- -D warnings
cargo check --workspace
cargo test --workspace
```
