nsc-work-man/src — everything that talks to the world
=====================================================


WHAT THIS CRATE IS FOR

  Fetching the most recently FINISHED candle, drawing a card, and sending it
  to Telegram. That is the whole program today.

  No analysis, no rules, no storage. Those come later and each has its own
  step in PROGRESS.md.


THE FILES

  WHAT THE BOT KNOWS IS NOT HERE. A candle, a level and what went wrong live
  in nsc-core, which has no reqwest and no tokio and so cannot reach anything.
  This crate is where reaching happens.


THE FILES

  main.rs       The flow, and nothing else. Read this first — it is short on
                purpose, and every line of it hands off to one of the others.

  feed.rs       Asking Twelve Data. One request, one answer.

  retry/        Doing a job again when the trouble says it is worth it. Here
                and not in nsc-core because it SLEEPS — waiting is doing.

  card/         Filling in an HTML template and letting Chrome screenshot it.
                A folder because it holds three jobs — putting the numbers in,
                turning candles into numbers, and driving Chrome — and because
                it has tests.

  telegram.rs   Sending. Several pictures go as one media group so the phone
                buzzes once and each picture still opens on its own.

  review.rs     Drawing a pair's levels, so he can see where they landed.

  watch/        THE BOT. Rungs 1 and 2 — price reaching one of his zones, and
                what a candle there did. The calendar, the heartbeat, and the
                line that has to survive being dropped.

  inbox/        The other side of Telegram. Listens for the levels he sends,
                and for /remove. RUNS INSIDE THE BOT, beside the watcher.

  bin/          Small programs that are NOT the bot. cards/ draws any card on
                demand, levels.rs draws a pair, listen.rs is the raw price
                stream kept as proof.

  README.txt    This file.


HOW THEY CONNECT

  main.rs is four lines. It calls watch::run, and that is the whole bot.

    watch::run
      -> inbox::run            spawned beside it — levels from his phone
      -> bands::for_pair       size every band, once, at startup
      -> the websocket         prices, free, about one a second
         -> watch::prices      is price at a zone? -> say::alert
         -> watch::closes      what did the candle do? -> say::closed
         -> watch::pulse       said nothing today? -> the heartbeat

  Only the first line costs requests unless price is actually at a zone.

  main.rs used to be step one — fetch gold's last hourly candle, draw a card,
  send it, every time it ran. That is the opposite of silence-by-default, and
  it was what the obvious command did.


THE RULE THAT RUNS THROUGH ALL OF IT

  The newest candle from the feed is almost always the hour still running.
  Its high is not its high and its close is not its close.

  It must never be drawn, quoted or analysed — and "drawn" is not an
  exception. The first version of the card took its headline price from that
  candle, and a wrong price on a picture is believed exactly like a wrong
  number in a table.

  Which candle has finished is asked of the CLOCK, never of position in the
  list. Position is right most of the time, which is worse than being wrong
  always, because you stop checking.


WHAT IS NOT HERE YET

  Any pair but gold. Any timeframe but the hour. Storage. The price watcher.
  The rules. See PROGRESS.md.
