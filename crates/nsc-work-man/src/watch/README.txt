watch/ — the price watcher
==========================


WHAT THIS IS FOR

      cargo run -p nsc-work-man

  Holds the price line open for every pair that has a levels file, and says
  what happens at his zones.

  TWO MESSAGES, AND NOTHING ELSE:

      a candle BREAKS a level    came from below and closed above, or
                                 came from above and closed below
      a shape he trades          at a level, or within half a band of one

  Silence is the normal state, and much more of it than there used to be.
  Price REACHING a zone was rung 1 until 1 September 2026; he asked it go.
  Prices still come down the line about once a second and none of them says
  anything now.


WHY IT IS NOT A BINARY

  It is library code, and src/main.rs is four lines that call run().

  main.rs used to be a leftover from step one: fetch gold's last hourly candle,
  draw a card, send it — EVERY TIME IT RAN, regardless of anything. That is the
  exact opposite of the rule the whole design rests on, and it was what `cargo
  run -p nsc-work-man` did.

  The obvious command now runs the real thing.


THE FILES

  mod.rs      The front door.

  kit.rs      What the watcher carries, and what has to survive the line
              dropping.

  run/      THE SUPERVISION LOOP. Loading, the calendar, and what to do
              when the line ends. Start here.

  line/     Holding the price line open, and everything that can end it —
              including a pair IBKR quietly refuses.

  bands.rs    Sizing a pair's bands, once, at startup.

  breathe.rs  Waiting between requests. IBKR allows 60 in ten minutes, and
              going over does not get refused -- it gets SLOWER.

  prices.rs   Every price off the line. Remembers the latest one and sends
              nothing, ever. Rung 1 lived here until 1 September 2026.

  reload.rs   Noticing he has sent a level, without being restarted.

  closes/   Rung 2 — what a candle at a zone is doing, and what it did.

  news/       THE ECONOMIC CALENDAR. A card five minutes before a rate
              decision or a payrolls number. It runs on its own beside the
              price watcher, because it needs no prices and no IBKR -- only
              the clock and the internet.

  pulse.rs    The heartbeat. Remembers when anything was last said, so a busy
              day stays quiet.

  resumed/  What to say when watching starts again — what it FOUND, never
              what arrived.

  say.rs      Drawing a card and sending it.

  standing.rs A copy of the live picture, published for the inbox to read when
              he asks /status.

  trouble.rs  Telling him when something has gone wrong, and when it is
              fixed.

  README.txt  This file.


A CARD THAT WILL NOT SEND IS NOT THE PRICE LINE BREAKING

  Every send now goes through keep_trying, which SendError has known how to
  answer since the day it was written — a dropped connection waits and tries
  again, a wrong token stops on the first go. It was simply never asked.

  And after three goes, a failure is logged and the run carries on. Letting it
  out dropped a perfectly good line and told him the feed was down.

  NOTHING IS MARKED AS SAID UNTIL IT HAS ACTUALLY GONE. That was wrong in four
  places at once, and each one lost something for good:

    the close      the candle was remembered as reported and never retried
    the greeting   the session was marked greeted with nothing sent, and it
                   is greeted once
    the heartbeat  marked beaten, so the day it was most needed was the day
                   it stayed quiet

  Half-sent counts as not sent. A repeat on the next look is far better than
  the one that failed never arriving.

  BUT NOT RETRIED ON EVERY PRICE. The waking report is asked for on every
  price that arrives, and prices arrive about once a second — so leaving it
  unmarked turned one failed send into a request a second at Telegram for as
  long as it stayed broken. It waits a minute between goes.


A CLOSE CARD IS ONCE PER BREAK, NOT ONCE PER CANDLE

  It was once per candle that touched the zone, which on a level price sat in
  for ten hours was ten cards. Since 31 August a close card goes out only when
  a candle BREAKS the level -- opened one side of it and closed the other.

  Price sitting in a zone, wicking into it, being thrown back out: silent. If a
  shape he trades printed there, rung 3 sends it, and that card names the shape
  rather than just saying the candle ended below.

  CLOSES ARE REPORTED ON EVERY LEVEL HE DREW, not the ones price is standing
  on. A break is price LEAVING a zone, so by the time the poll runs it has
  often gone -- and reading the resting list dropped exactly the cards worth
  sending.


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

  It is checked on the same tick as the closes — at most ten minutes, sooner
  when a candle is due — and the tick is pushed forward BEFORE anything decides
  to skip the check. Left until after the work, a Monday leaves the deadline in
  the past and the loop spins as fast as the processor will go.


IT WATCHES NOTHING ON A MONDAY

  Not "stays quiet" — NOTHING. No prices checked, no candles fetched, no queue
  building up to be dumped on him on Tuesday morning.

  The line is not opened at all. There is nothing to drain, because nothing
  was subscribed to.

  See nsc-core::when. Sunday evening is already Monday's session, which is not
  something the UTC calendar knows.


WHAT IT COSTS

  Startup: one request per pair per timeframe he has levels on. Four pairs
  across three timeframes would be twelve; skipping the timeframes with no
  levels makes it seven.

  After that: NOTHING, unless price is actually at a zone. Prices come down
  the subscriptions for free — IBKR charges nothing for streaming quotes on a
  pair the account is entitled to.

  Rung 2 only asks about pairs with a live zone. A quiet week costs nothing.
  Gold sitting in a zone all day costs about 24 requests.

  THE LIMIT THAT SHAPES IT: 60 historical requests in any ten minutes. That is
  one every ten seconds sustained, which is why BREATHE is ten seconds.

  Go over it and IBKR does not refuse. IT PACES — the request just takes
  longer, and then longer, and a candle report arrives late enough to be about
  a candle he has already watched close on his own screen.

  Every request goes through a 7.5-second gap, which is 8 a minute exactly.


THE REST IS IN THE FOLDER IT IS ABOUT

  This file got long enough that nobody would read it end to end, which is the
  exact thing the 250-line rule exists to stop. So the detail moved down to
  the folder it describes:

    closes/README.txt    the twenty-minute look, and why it NEVER works out
                         when a candle closes for itself

    run/README.txt       what must never stop the bot -- the four things that
                         used to, and none of them the price line breaking.
                         And how a level he sends is picked up mid-run

    resumed/README.txt   the opening hours: watched, not spoken about. And
                         what Tuesday says about a move made on Monday

    line/README.txt      why the greeting is asked AFTER the price, and what
                         happened for a whole session when it was not
