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

  bin/          Small programs that are not the bot. inbox/ listens for
                levels, listen.rs watches the live price stream, levels.rs
                draws a pair on demand.

  README.txt    This file.


HOW THEY CONNECT

  main.rs
    -> feed::candles          ask for the last 120 hourly candles
    -> Bar::is_finished       drop the hour still running
    -> message::build         the caption
    -> card::render           the picture
    -> telegram::send         out


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
