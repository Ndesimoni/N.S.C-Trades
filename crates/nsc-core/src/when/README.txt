when/ — when the bot is allowed to speak
========================================


WHAT THIS FOLDER IS FOR

  Answering one question: at this moment, may the bot say anything, may it
  report but not suggest a trade, or should it be entirely quiet?


THE FILES

  mod.rs      The front door.

  rules.rs    Reading config/when.toml.

  session.rs  WHICH TRADING DAY A MOMENT BELONGS TO. The hard part. Start
              here.

  allow.rs    The three states, and the decision.

  beat.rs     THE HEARTBEAT — whether it is due, and what it says.

  tests.rs    Fifteen tests, most of them on the boundary.

  README.txt  This file.


THE TRADING WEEK IS NOT THE CALENDAR WEEK

  It opens SUNDAY 17:00 NEW YORK. So the session everyone calls Monday runs
  from Sunday evening to Monday evening.

  Read a moment off the UTC calendar instead and both halves go wrong at once:

      Sunday 22:00 UTC   "Sunday"   really Monday   — spoken through, wrongly
      Monday 22:00 UTC   "Monday"   really Tuesday  — silenced, wrongly

  He does not trade Monday and does trade Tuesday, so that mistake talks
  during the session he ignores and shuts up during the one he works.

  Nothing errors. It is just wrong, every week.


AND 17:00 NEW YORK IS NOT A FIXED UTC TIME

      summer   21:00 UTC
      winter   22:00 UTC

  Written in the config as a UTC time it would be right for half the year and
  an hour out for the rest — which is why the config holds "17:00" and
  "America/New_York" and never a UTC clock time.

  That is what chrono-tz is in nsc-core for. It is pure data, no network and
  no async, so it does not break the rule that this crate cannot reach.


THREE STATES, NOT TWO

      Anything     everything, including a trade
      WatchOnly    what is happening, but no trade suggested
      Silence      nothing but the heartbeat

  "Do not trade" and "do not speak" are different things. Collapse them and
  you either silence a day he wants to watch, or suggest trades in the hours
  he never takes them.

  Price reaching a zone and a candle closing in one are INFORMATION. A signal
  is an INSTRUCTION. The settle window holds back the instruction and lets the
  information through.


WHAT EACH RULE IS FOR

  SILENT DAYS — Monday. And it means nothing at all: no prices checked
  against bands, no candles fetched, no queue building up to be dumped on him
  on Tuesday morning. If we are not trading Monday there is nothing worth
  collecting on Monday.

  THE HEARTBEAT STILL GOES OUT, or a quiet Monday and a dead bot look exactly
  the same.

  SETTLE HOURS — four, measured from the session's own open. The first hours
  of a day are where a move gets faked and taken back.

  Four rather than five because it is one 4-hour candle, so the window ends on
  a boundary that exists.

  NO NEW TRADES — Friday. A setup that needs the weekend to work out is one
  nobody can manage, and Sunday's gap is not something a stop protects
  against.


NOTHING HERE READS THE CLOCK

  Every function is handed `now`. That is what lets the backtester run these
  exact rules over 2019 by passing 2019 in, with no "if we are backtesting"
  anywhere — which is the whole reason nsc-core is shaped the way it is.


THE HEARTBEAT, AND WHY IT IS CONDITIONAL

  ONE LINE, AND ONLY ON A DAY THAT SAID NOTHING ELSE.

  Silence is the default everywhere in this bot, which leaves exactly one
  problem: after a quiet day he cannot tell whether nothing happened or the
  bot died.

  On a busy day it never fires. A message already arrived; he knows.

  IT GOES OUT AT 07:00 UTC, before London opens. The point is to know the bot
  is working BEFORE the hours he trades — at the end of the day it would be a
  post-mortem.

  That time is in UTC, unlike day_ends. This one is about when he looks at his
  phone, not about the market, and it does not move with the seasons.

  DUE AT THE FIRST 07:00 AFTER THE SESSION OPENED, not "today at 07:00". The
  session runs 17:00 New York to 17:00 New York, so it straddles midnight —
  read off the calendar day it would come due before the session it reports on
  had begun.

  ONCE A SESSION. A heartbeat that repeats is worse than none, because he
  stops reading it, and that is the one thing it cannot survive.

  AND IT FIRES ON MONDAY. Monday watches nothing, so without this a quiet
  Monday and a dead bot look exactly the same — which is the whole reason it
  exists.


WHAT TUESDAY MORNING CANNOT KNOW

  Price can walk into a zone during Monday's silence and still be sitting
  there when Tuesday opens. Nobody watched it happen, so nobody can say when.

  The watcher reports that as FOUND rather than ARRIVED — see
  levels::alert::News. A card saying "arrived" would put a Monday move on a
  Tuesday clock.
