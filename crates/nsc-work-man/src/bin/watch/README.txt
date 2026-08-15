watch/ — the price watcher
==========================


WHAT THIS PROGRAM IS FOR

      cargo run -p nsc-work-man --bin watch

  Holds the price line open for every pair that has a levels file, and says
  what happens at his zones.

  RUNGS 1 AND 2 OF THE LADDER:

      price reaches a zone       an alert card, ONCE PER VISIT
      a candle closes there      a card PER CANDLE that touched it

  Silence is the normal state. Prices arrive about once a second and almost
  all of them are nowhere near anything.


THE FILES

  main.rs     Loading, the calendar, the socket, and the two things that can
              happen joined together. Start here.

  bands.rs    Sizing a pair's bands, once, at startup.

  prices.rs   Every price off the line, against the bands. Says nothing on the
              overwhelming majority of them, which is the point.

  closes.rs   Rung 2 — what a candle at a zone is doing, and what it did.

  pulse.rs    The heartbeat. Remembers when anything was last said, so a busy
              day stays quiet.

  say.rs      Drawing a card and sending it.

  README.txt  This file.


THE TWO RUNGS FIRE ON DIFFERENT RULES, ON PURPOSE

  RUNG 1 IS ONCE PER VISIT. Prices come once a second and barely move. Without
  that rule one visit to a level becomes twenty alerts and he stops reading
  them.

  RUNG 2 IS ONCE PER CANDLE. That is his own decision and it is deliberate:
  while price is at a zone he wants to watch it candle by candle, not be told
  once and left guessing. Ten hours in a zone is ten cards.

  Silence resumes the moment a candle does not touch the zone.


THE ONE MESSAGE THAT STILL GOES OUT ON A MONDAY

  The heartbeat. A card at 07:00 UTC, and only on a day that said nothing
  else: every pair, its levels as dots in his colours, and how far price is
  from the nearest zone on each.

  Monday watches nothing, so without it a quiet Monday and a dead bot look
  exactly the same.

  It is checked on the same ten-minute tick as the closes — and the tick is
  pushed forward BEFORE anything decides to skip the check. Left until after
  the work, a Monday leaves the deadline in the past and the loop spins as
  fast as the processor will go.


IT WATCHES NOTHING ON A MONDAY

  Not "stays quiet" — NOTHING. No prices checked, no candles fetched, no queue
  building up to be dumped on him on Tuesday morning.

  The socket is still drained so it does not back up. Nothing on it is looked
  at and nothing is remembered.

  See nsc-core::when. Sunday evening is already Monday's session, which is not
  something the UTC calendar knows.


WHAT TUESDAY SAYS

  Price can walk into a zone during Monday's silence and still be there when
  Tuesday opens. The watcher fires on a CHANGE, so nothing would ever be said
  about it.

  So the first thing after silence is a report of what was FOUND. The card
  says "already in the zone", not "arrived" — because nobody watched it happen
  and an arrival card would put a Monday move on a Tuesday clock.


WHAT IT COSTS

  Startup: one request per pair per timeframe he has levels on. Four pairs
  across three timeframes would be twelve; skipping the timeframes with no
  levels makes it seven.

  After that: NOTHING, unless price is actually at a zone. Prices come down
  the socket for free.

  Rung 2 only asks about pairs with a live zone. A quiet week costs nothing.
  Gold sitting in a zone all day costs about 24 requests against a limit of
  800.

  Every request goes through a 7.5-second gap, which is 8 a minute exactly.


THE TWENTY-MINUTE LOOK

  A candle at one of his zones gets spoken about TWICE:

      about a third of the way in    what it has done SO FAR
      when it finishes               what it did

  The first is the only place in this project that reads a candle before it
  has finished. It is allowed for the same reason the price alert is — it is a
  heads-up and nothing more — and the card says so on its own face. IT MUST
  NEVER REACH A STRATEGY.

  NOT ON THE OPEN. Spot forex runs Sunday to Friday without a break, so a
  candle's open IS the last one's close. That message would repeat what
  arrived a minute earlier.

  ONE REQUEST SERVES BOTH. The reply already carries the candle that just
  finished and the one still running, so the look costs nothing on top.

  The two are remembered apart. Keyed together, the look would silence the
  close that follows it — the one that actually matters.

  `look_in_minutes` in config/when.toml is set for the 1-hour and scaled for
  the rest: twenty minutes into an hour is eighty into a 4-hour.

  The check runs every ten minutes, so the look lands somewhere between twenty
  and thirty minutes in. Close enough for a heads-up; nothing decides anything
  on it.


IT NEVER WORKS OUT WHEN A CANDLE CLOSES

  closes.rs asks every ten minutes for the newest candle and lets THE FEED'S
  OWN STAMP say whether it is one already reported.

  Working the boundaries out here would mean knowing where the feed puts its
  4-hour candles, which nobody has measured. Guessing wrong reports a candle
  that has not finished — and reading a candle early does not error, it makes
  results look better.

  A 4-HOUR CANDLE DOES NOT EXIST UNTIL ITS LAST HOUR HAS CLOSED. Three hourly
  closes can pass with the 4-hour still saying nothing; the fourth is when it
  speaks. Bar::finished_by is the single place that decides.
