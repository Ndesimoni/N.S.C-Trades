ci/
====

The checks that run on every push, and that you can run yourself before you
push.

  rules.sh    the project-specific checks. Not general code review — these
              are the nine ways this system breaks quietly.

Run it any time:

    ./ci/rules.sh

It needs nothing installed. No cargo, no network. It reads files and greps.
That is deliberate: it finishes in under a second, so you can run it as often
as you like.


Why a separate script instead of only a workflow file
-----------------------------------------------------

A check you can only run by pushing is a slow check. You push, wait three
minutes, read a red tick, fix one line, push again. Running the same script
locally turns that into a second.

`.github/workflows/ci.yml` runs this exact file. There is one copy of the
rules, so CI and your machine can never disagree about them.


What it checks, and what breaks without each one
------------------------------------------------

1.  Clean crates have no dirty dependencies.
    nsc-ta and nsc-strategy must never gain tokio, sqlx, reqwest, redis and
    friends. Those two crates are what lets the backtester and the live bot
    run the same code. The moment one of them can reach the internet, they
    are two different systems and your backtest stops predicting anything.

2.  Clean crates do not read the clock or hold global state.
    Catches Utc::now, SystemTime::now, async fn, static mut, OnceLock,
    std::env. A rule that reads the wall clock gives a different answer when
    replayed than it did live, and nothing about the output tells you so.

3.  No "am I backtesting?" branches.
    Catches is_backtest, in_backtest, backtest_mode. This is the single
    change that breaks the whole design. It also always makes backtests look
    better, never broken, so you will not catch it by reading results.

4.  Crates only depend downward.
    nsc-core depends on no other nsc crate. nsc-ta depends on nsc-core only.
    The clean crates never reach down into the messy ones.

5.  No unwrap, expect, panic!, todo! or unimplemented! in library crates.
    Test files are exempt. A settings sweep runs this code over years of
    candles; one bad candle must not throw away two hours of work.

6.  No .rs file over 250 lines.
    Past that a file is a thing you scroll, not a thing you read. The bugs
    that hurt here are quiet ones — a boundary an hour out, a candle read one
    early — and quiet bugs hide in files nobody reads end to end.

7.  mod.rs is a front door, not a room.
    No struct, enum, fn, impl or trait in a mod.rs. One screen should tell
    you what a folder holds without opening anything.

8.  No hardcoded pip numbers outside config/.
    20 pips is a big move on EURUSD and a small one on GBPJPY. A pip number
    baked into code works on the pair you tested it on and quietly stops
    working on every other one.

9.  No secrets tracked by git.
    Blocks .env and .pem files. Once a key is in a commit it is in the
    history forever, even if the next commit deletes it.


Two checks that are deliberately NOT here
-----------------------------------------

Both were written, tested against the real tree, and thrown away. A check
that cries wolf is worse than no check, because you learn to click past the
red tick — and then you click past a real one.

  "Every folder with code has a README.txt."
  CLAUDE.md exempts folders of empty stubs, and most of this repo is empty
  stubs right now. No script can tell a stub from real code, so this one
  would have flagged about twenty folders that are correct as they stand.

  "No README.txt names a file that no longer exists."
  READMEs legitimately name files in other folders (crates/nsc-core/src/
  candle/README.txt points at ../error.rs) and files in subfolders. Match
  only the folder itself and you get nine false alarms. Search the whole
  crate instead and you match any same-named file anywhere, which lets the
  real staleness through. Neither setting is worth having.

Both of those stay human jobs — see the merge-check skill.


When the database arrives
-------------------------

Phase 0 brings sqlx and migrations. sqlx's compile-time checked queries need
a live Postgres at build time, so ci.yml will need a `services: postgres`
block and a DATABASE_URL, or the queries switched to the offline `.sqlx`
cache. Nothing is stubbed in for it now, because pipe work written before it
is needed rots before it is used.
