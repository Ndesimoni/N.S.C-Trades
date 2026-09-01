watch/news/ — telling him what is about to print
================================================


WHAT THIS FOLDER IS FOR

  A card five minutes before a rate decision, a CPI number or a payrolls
  release, and a second one a minute before it, so a level sitting in front of
  one is not read the same way as a level on a quiet Thursday.

  IT WAS THIRTY MINUTES UNTIL 1 SEPTEMBER 2026. His call after a day of it:
  "thirty minutes is really fast." A warning that far ahead is read, filed and
  forgotten before the number prints -- it costs a buzz and buys nothing.


THE FILES

  mod.rs       The front door.
  holding.rs   What it has downloaded, and what it has already said.
  saying.rs    Drawing the card and sending it.
  README.txt   This file.


IT RUNS ON ITS OWN, BESIDE THE PRICE WATCHER

  Spawned once at startup, like the inbox.

  It needs no prices, no bands and no IBKR -- only the clock and the internet.
  Putting it inside the price loop would have been wrong anyway: that loop
  blocks for hours at a stretch waiting on the socket, so a check living
  inside it would only run when the line dropped.


IT FAILS QUIET, NEVER LOUD

  The calendar going down must not stop the bot.

  Every failure is printed and the watcher carries on with whatever it
  downloaded last. Clearing the list on a refused download would turn one bad
  afternoon into a silent one -- and a silent afternoon looks exactly like a
  quiet week.

  A missing or unreadable config/news.toml does not stop the bot either. It
  says so and everything else runs. Saying what price is doing at his levels
  is the job; the calendar is an addition to it.


ONCE PER RELEASE, AND IT HAS TO SURVIVE A RE-READ

  The week's file is downloaded every few hours and the same event is in every
  copy of it. So "already said" is kept by Event::key -- time, currency and
  title together -- rather than by position in the list.

  The time alone would not do: three Australian CPI numbers print in the same
  second. The title alone would not either: the same release comes round every
  month.

  WHAT IS SAID IS MARKED ONLY ONCE IT HAS GONE. Marked first, one failed send
  loses the warning for good. That is the mistake the heartbeat made, where
  marking it early silenced it for a whole day.

  The list of what has been said is pruned on every download to whatever is
  still in the file. It empties itself when the week rolls over, with nothing
  to schedule.


CHROME RUNS OFF THE LOOP

  Drawing a card is a blocking wait of two to ten seconds. saying.rs sends it
  to spawn_blocking, which has a thread pool for exactly this.

  The six older cards still block a worker and that is the top open bug in
  this project. This one does not, because it was written after the bug was
  understood -- there was no reason to add a seventh.
