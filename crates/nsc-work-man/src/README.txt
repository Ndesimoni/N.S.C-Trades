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

  retry/        Doing a job again when the trouble says it is worth it. Here
                and not in nsc-core because it SLEEPS — waiting is doing.

  web.rs        THE ONE HTTP CLIENT, and the timeouts on it. It was
                reqwest::Client::new(), which sets none at all, so a request
                that hung hung forever — and retry/ cannot save that, because
                retrying answers an error and a hang is not an error.

  places.rs     WHERE THINGS ARE — his inbox, the settings files, and where
                cards are drawn. One place, because his chat id was written
                out in three files and config/pairs in five. None of them
                disagreed, which is the only reason nobody noticed: two
                copies of a string agree right up until one is changed.

  secrets.rs    Reading .env, and SAYING SO when it will not read. dotenvy
                stops at the first bad line and loads nothing after it, and
                .ok() throws the reason away — so one unquoted value takes out
                every setting below it in silence.

  card/         Filling in an HTML template and letting Chrome screenshot it.
                A folder because it holds three jobs — putting the numbers in,
                turning candles into numbers, and driving Chrome — and because
                it has tests.

  telegram/     Sending. Several pictures go as one media group so the phone
                buzzes once and each picture still opens on its own.

  review/       Drawing a pair's levels, so he can see where they landed, on
                whichever of his three charts he asked for. A folder because it
                holds a type — how much of what he drew is actually on the
                picture — and the tests that pin it.

  watch/        THE BOT. Rungs 1 and 2 — price reaching one of his zones, and
                what a candle there did. The calendar, the heartbeat, and the
                line that has to survive being dropped.

  inbox/        The other side of Telegram. Listens for the levels he sends,
                and for /remove. RUNS INSIDE THE BOT, beside the watcher.

  bin/          Small programs that are NOT the bot. cards/ draws any card on
                demand, levels.rs draws a pair, listen.rs prints IBKR's raw
                ticks so you can see what actually arrives.

  README.txt    This file.


HOW THEY CONNECT

  main.rs is four lines. It calls watch::run, and that is the whole bot.

    watch::run
      -> IbkrConnection        the one line to TWS, opened at startup
      -> inbox::run            spawned beside it — levels from his phone
      -> bands::for_pair       size every band, once, at startup
      -> ibkr.prices           one subscription per pair, folded into one
         -> watch::prices      remembers the latest price, says nothing
         -> watch::closes      what did the candle do? -> say::closed
         -> watch::pulse       said nothing today? -> the heartbeat

  Only watch::closes costs requests, and `due` holds each pair and
  timeframe until its next candle is actually due.


WHERE PRICES COME FROM

  NOT FROM THIS CRATE. Everything is IBKR, and IBKR lives in nsc-data.

  Nothing here holds a broker's address, a key, or a wire format. The `feed/`
  folder that used to ask Twelve Data was deleted on 20 August 2026 when the
  feed changed, and that was the whole change on this side.

  WHAT IBKR COSTS, WRITTEN DOWN SO IT IS NOT A SURPRISE:

    - TWS OR IB GATEWAY MUST BE RUNNING AND LOGGED IN. There is no feed at
      all without it, and no fallback.

    - Gold needs its own market data subscription. Spot metals are not spot
      forex at IBKR, and an account can have one and not the other.

    - A price is the MIDDLE of the bid and the ask, worked out in nsc-data.
      It has to be, because the candles come back as mid prices.

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
