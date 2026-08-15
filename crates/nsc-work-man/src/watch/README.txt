watch/ — the price watcher
==========================


WHAT THIS IS FOR

      cargo run -p nsc-work-man

  Holds the price line open for every pair that has a levels file, and says
  what happens at his zones.

  RUNGS 1 AND 2 OF THE LADDER:

      price reaches a zone       an alert card, ONCE PER VISIT
      a candle closes there      a card PER CANDLE that touched it

  Silence is the normal state. Prices arrive about once a second and almost
  all of them are nowhere near anything.


WHY IT IS NOT A BINARY

  It is library code, and src/main.rs is four lines that call run().

  main.rs used to be a leftover from step one: fetch gold's last hourly candle,
  draw a card, send it — EVERY TIME IT RAN, regardless of anything. That is the
  exact opposite of the rule the whole design rests on, and it was what `cargo
  run -p nsc-work-man` did.

  The obvious command now runs the real thing.


THE FILES

  mod.rs      The front door, and the few things every file here shares.

  run.rs      Loading, the calendar, the socket, and the two things that can
              happen, joined together. Start here.

  bands.rs    Sizing a pair's bands, once, at startup.

  prices.rs   Every price off the line, against the bands. Says nothing on the
              overwhelming majority of them, which is the point.

  reload.rs   Noticing he has sent a level, without being restarted.

  closes.rs   Rung 2 — what a candle at a zone is doing, and what it did.

  pulse.rs    The heartbeat. Remembers when anything was last said, so a busy
              day stays quiet.

  say.rs      Drawing a card and sending it.

  trouble.rs  Telling him when something has gone wrong, and when it is
              fixed.

  README.txt  This file.


THE TWO RUNGS FIRE ON DIFFERENT RULES, ON PURPOSE

  RUNG 1 IS ONCE PER VISIT. Prices come once a second and barely move. Without
  that rule one visit to a level becomes twenty alerts and he stops reading
  them.

  RUNG 2 IS ONCE PER CANDLE. That is his own decision and it is deliberate:
  while price is at a zone he wants to watch it candle by candle, not be told
  once and left guessing. Ten hours in a zone is ten cards.

  Silence resumes the moment a candle does not touch the zone.


WHEN SOMETHING BREAKS, HE HEARS ABOUT IT

  QUIET ABOUT HICCUPS, LOUD ABOUT OUTAGES.

  The price line drops. It always will — the feed closes an idle one, the wifi
  blinks, a router reboots. Almost all fix themselves in seconds, and a
  message for each is the same mistake as a candle every hour: he learns the
  buzz means nothing, and then ignores the one that meant something.

  IT GOES AS A CARD, like everything else he is sent, with the severity in the
  colour: amber the line is down and it is trying, green it is back, red it
  has stopped and will not restart. Red is the only one that asks him to do
  anything.

  So nothing is said until it has been down `trouble_after_minutes` — five —
  and then it is said ONCE. With a second message when it comes back, because
  "it broke" on its own leaves him checking his phone all evening.

  THREE THINGS IT WILL NOT LET PASS QUIETLY:

    the line keeps failing      after five minutes, one message
    the line opens and SHUTS    without a single price. A key over its quota
                                does exactly that, and returning Ok for it
                                would reconnect forever in silence
    it cannot recover at all    a key it will never be given, a config that
                                will not parse. It says so and stops

  The last one is main.rs, not here. run() returning Err means give up, and
  the last thing it does is tell him — because from his side a bot that
  stopped looks exactly like a market where nothing happened.

  TROUBLE COUNTS AS HAVING SPOKEN. He heard from the bot, so he knows it is
  alive, which is all the heartbeat was going to tell him.


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


A LEVEL SENT WHILE IT IS RUNNING IS PICKED UP

  The levels used to be read ONCE, at startup. He would send one from his
  phone, the inbox would save it correctly, the file would be right — and the
  watcher would never look again. Nothing said so. The level simply did
  nothing until the next restart, which might be days.

  Now the folder is checked every ten minutes, BY THE CLOCK ON THE FILES.
  Parsing every pair file to find out that nothing happened is work done for
  nothing, and nothing is the normal answer. The count is checked too — a file
  deleted leaves every remaining timestamp exactly as it was.

  A PAIR WHOSE LEVELS ARE UNTOUCHED KEEPS THE WATCH IT ALREADY HAD. Rebuilt,
  it would forget which zones price is sitting in and announce every one of
  them again as though it had just arrived. Only the changed pair costs a
  request.

  The line is then closed and opened again, because THE SUBSCRIPTION IS FIXED
  WHEN THE SOCKET OPENS — a pair added to a live one would never be asked
  about. No thirty-second pause on that path; he is standing there having just
  sent it.

  IT ALSO HAPPENS ON QUIET DAYS. The weekend is exactly when he does his chart
  work, and the check lived inside the socket loop at first — which does not
  run on a quiet day. A level sent on Sunday sat unarmed until Tuesday.

  He gets a card back: a tick, "Got it. Your levels are live", and the count
  of what is being watched.

  NO PAIR NAMES. He has just sent it — he knows what he sent, and
  the inbox has already sent back a picture of where the bands landed, with
  the pair on it in his colours. Repeating that is a second message telling
  him something he already had.

  What that picture cannot say is that they are being WATCHED. Saved and armed
  were two separate states and nothing told him which one he had. That is the
  whole job of this card.

  The count is the one detail worth having: he sees the number went up.


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
