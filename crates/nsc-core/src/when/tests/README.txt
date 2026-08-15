tests/ — when the bot may speak
===============================


THE FILES

  mod.rs        The front door.

  boundary.rs   WHICH TRADING DAY A MOMENT BELONGS TO. The only hard thing in
                this folder, and where a mistake would hide.

  allowing.rs   The three states — the settle window, Friday, a normal day.

  beating.rs    The heartbeat: when it is due, and when it stays quiet.

  support.rs    The calendar these tests run against, and a shorthand for
                writing a moment.

  README.txt    This file.


WHY support.rs WRITES THE CALENDAR OUT

  It does not read config/when.toml.

  A test that loads the real file passes or fails depending on what he changed
  this morning, and then it is testing his settings rather than the rules.
  These check the RULES; the settings are his to move.


THE TWO THAT MATTER MOST

  boundary.rs::sunday_evening_belongs_to_mondays_session
  boundary.rs::monday_evening_belongs_to_tuesdays_session

  They are two halves of the same mistake. The forex week opens Sunday 17:00
  New York, so read off the UTC calendar the bot would talk right through the
  session he ignores and go silent through the one he works.

  Nothing would error. It would just be wrong, every week.

  And the_boundary_follows_new_york_through_the_clock_change is the same
  mistake spread over the year: 17:00 New York is 21:00 UTC in summer and
  22:00 in winter, so a fixed UTC boundary is right for half of it.
