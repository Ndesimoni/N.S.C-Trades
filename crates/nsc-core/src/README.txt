nsc-core/src — what the bot knows
=================================


WHAT THIS CRATE IS FOR

  A candle. A level. What went wrong, and whether it is worth another go.

  THERE IS NO FEED IN HERE, NO TELEGRAM, NO CHROME AND NO CLOCK — and not by
  discipline. Cargo.toml has no reqwest and no tokio, so nothing in this crate
  CAN reach anything. The compiler refuses.

  Tried, to be sure:

      error[E0433]: failed to resolve: use of unresolved module
                    or unlinked crate `reqwest`


WHY THAT MATTERS MORE THAN IT SOUNDS

  The analysis lives here — swings, structure, and eventually the strategies.

  The backtester and the live bot have to run THE SAME analysis and get THE
  SAME answer. That only holds if the analysis physically cannot fetch
  anything, because the moment it can, a backtest stops describing the bot.

  And it does not look broken when that happens. It looks BETTER. That is
  what makes it dangerous, and why the boundary is a manifest rather than a
  rule people remember.

  If this Cargo.toml ever gains reqwest, tokio or sqlx, the change is wrong.


THE FOLDERS

  candle/     One candle, and the only question that matters about it: has it
              finished?

  levels/     The lines he drew, and the bands they become.

  error/      Everything that can go wrong, and the one question each answers:
              try again, or give up?

  settings.rs What to fetch, until step 2 moves it into config/.

  message.rs  The one line under a picture — the notification banner.


WHERE THE LINE FELL, AND WHO DECIDED IT

  The compiler did, when the crates were split. Three things would not build
  in here, and each one was right to be thrown out:

    keep_trying     it SLEEPS. Waiting is doing, not knowing — it went to
                    nsc-work-man/src/retry/.

    anyhow          a program with a person watching it needs a trail. A
                    library needs a decision. Bar::opened_at now returns
                    CandleError, which says GIVE UP: a stamp that cannot be
                    read now cannot be read in three seconds either.

    message::build  it returned a Result and could not fail. It returns a
                    String.

  None of those were noticed by reading. All three were found in about a
  minute by moving the code somewhere it could not cheat.
