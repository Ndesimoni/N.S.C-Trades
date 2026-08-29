news/tests/ — twenty tests
==========================


THE FILES

  mod.rs      The front door.
  support.rs  The moment every test uses, an event, and the shipped settings.
  reading.rs  Reading the feed's word, and matching it to his settings.
  window.rs   THE TWO EDGES. The most important tests here.
  naming.rs   Naming an event once, and grouping a release onto one card.
  README.txt  This file.


THE ONE THAT WOULD HURT MOST IF IT BROKE

  window.rs -- a_release_from_this_morning_is_history_not_news.

  Delete the far edge of the window and nothing errors. The bot simply sends
  every release of the day the moment it restarts, all at once, all useless.
  It looks like a burst of activity rather than a bug.
