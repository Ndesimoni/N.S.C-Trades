src/ — the bot
==============


WHAT THIS FOLDER IS FOR

  Fetching the most recently FINISHED candle, drawing a card, and sending it
  to Telegram. That is the whole program today.

  No analysis, no rules, no storage. Those come later and each has its own
  step in PROGRESS.md.


THE FILES

  main.rs       The flow, and nothing else. Read this first — it is short on
                purpose, and every line of it hands off to one of the others.

  settings.rs   The pair, the timeframe, how many candles, how many decimals.
                All written into the code, which is honest for one pair and
                would be a mess for two. STEP 2 REPLACES THIS FILE with
                reading config/ — the project rule is that anything a trader
                would tune lives in a TOML file, and this is the one place
                that breaks it on purpose.

  candle/       One candle, and the only question that matters about it:
                has it finished? A folder rather than a file because it
                defines a type and it has tests, and this project's rule is
                that those two together earn a folder from the start.

  feed.rs       Asking Twelve Data. One request, one answer.

  card/         Filling in an HTML template and letting Chrome screenshot it.
                A folder because it holds three jobs — putting the numbers in,
                turning candles into numbers, and driving Chrome — and because
                it has tests.

  message.rs    The one line under the picture. It is the notification
                banner, not the message — the card is the message.

  telegram.rs   Sending. Several pictures go as one media group so the phone
                buzzes once and each picture still opens on its own.

  bin/          Small programs that are not the bot. listen.rs watches the
                live price stream.

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
