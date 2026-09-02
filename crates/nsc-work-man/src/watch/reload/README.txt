reload/ — noticing he has sent a level, without a restart
=========================================================


WHAT THIS FOLDER IS FOR

  He adds a level from his phone. The watcher notices and starts watching it,
  without the bot being restarted.

  Split out of reload.rs on 2 September 2026, when it went past the file
  limits.


THE FILES

  mod.rs        The front door.

  noticing.rs   Has anything changed? It watches the FOLDER, not a clock --
                how many pair files there are and the newest time any was
                touched.

  doing.rs      Reading the levels again, and keeping what has not changed.

                A PAIR WHOSE LEVELS ARE UNTOUCHED KEEPS THE WATCH IT HAD.
                Rebuilt, it would forget what each of its levels last
                reported, and announce it all over again.

                A PAIR THE FEED WILL NOT SIZE KEEPS THE BANDS IT HAD. This
                used to return an error that travelled all the way out and
                stopped the bot -- he sent a level, the feed was slow for ten
                seconds, and the bot said "stopped" and quit.

  tests.rs      The lookup that decides whether a pair keeps being watched.

  README.txt    This file.


THE BUG THE TESTS ARE ABOUT

  The watch list is keyed by SYMBOL -- "XAU/USD". The files are named without
  the slash -- "XAUUSD.toml". The branch handling an unreadable file looked
  the file name up in the watch list directly, so it could never hit, and the
  branch whose whole job is "keep watching what this pair had" dropped it
  instead -- while printing that it was leaving it alone.
