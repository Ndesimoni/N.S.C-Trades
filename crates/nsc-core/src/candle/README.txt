candle/ — one candle, and whether it has finished
=================================================


WHAT THIS FOLDER IS FOR

  Holding a candle exactly as the feed sends it, and answering the one
  question that decides whether this whole project can be trusted:

      HAS THIS CANDLE FINISHED?

  A candle still forming has a high that is not its high and a close that is
  not its close. Reading it is reading prices the market has not printed.


THE FILES

  mod.rs      The front door.

  bar.rs      Bar and Series. What the feed sends, and what can be asked of
              it.

  tests.rs    Eight tests. Read the last one in the first group first.

  README.txt  This file.


WHY IT IS A FOLDER AND NOT A FILE

  The project rule: a module that defines a type AND has tests is a folder,
  whatever its length. Splitting is cheap while there are three things in it
  and a nuisance once there are twelve.


THE ANSWER IS TAKEN FROM THE CLOCK, NEVER FROM THE LIST

  The feed sends the newest candle first, so the obvious rule is "skip the
  first one". That rule is wrong, and it is wrong in a way that hides.

  Ask Twelve Data at 18:00:02 and you get one of two things:

    a price has already landed in the new hour
        -> the newest is the 18:00 candle, and the finished one is SECOND

    no price has landed yet
        -> the newest is the 17:00 candle, now finished, and it is FIRST

  Position is right most of the time. That is worse than being wrong always,
  because you stop checking it.

  Test: which_candle_is_newest_and_which_is_finished_are_different_questions


THE STAMP MEANS TWO DIFFERENT THINGS

  An hourly stamp — "2026-08-14 17:00:00" — is when the candle OPENED.

  A daily stamp — "2026-08-14" — is the date the candle ENDED on. That candle
  started at 17:00 New York on the 13th.

  Same field, opposite meanings. opened_at REFUSES a daily stamp rather than
  guessing, because guessing would put every daily candle a whole day out.

  Test: a_daily_stamp_is_refused_rather_than_guessed_at
  Detail: docs/worksheets/twelve-data.md


NO VOLUME

  Spot forex and gold have no volume — there is no central exchange to count
  it. There is no field for it here and no rule in this project may ever use
  it.
