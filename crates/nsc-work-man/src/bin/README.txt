bin/ — small programs that are not the bot
==========================================


WHAT THIS FOLDER IS FOR

  Anything in here becomes its own program, run with:

      cargo run -p nsc-work-man --bin <name>

  They are separate from the bot because they have a different shape. The bot
  does one job and exits. These hold a line open, or answer one question and
  stop.


THE FILES

  watch/      THE PRICE WATCHER, and the closest thing here to the real bot.
              Holds the price stream open for every pair that has a levels
              file, and says what happens at his zones.

              Rungs 1 and 2 of the ladder. Silence is the normal state, and
              on a Monday it watches nothing at all. Has its own README.

  inbox/      Listens to Telegram and saves the levels he sends from his
              phone. Buttons rather than typing. Has its own README.

  levels.rs   Draws a pair's levels on its weekly chart and sends the
              picture. For checking that our band sits where his does.

                  cargo run -p nsc-work-man --bin levels -- GBPUSD

  cards/      Draws ANY card and sends it, without waiting for the market to
              do anything.

                  --bin cards -- XAUUSD                approaching
                  --bin cards -- XAUUSD 4120           in the zone
                  --bin cards -- XAUUSD 4120 found     already in
                  --bin cards -- XAUUSD close          a candle's close
                  --bin cards -- XAUUSD close 4375.6   ...against a made-up
                                                       level it actually met
                  --bin cards -- XAUUSD close 4375.6 sofar   still running
                  --bin cards -- heartbeat             the quiet-day card

              The design loop. Changing how a card looks means looking at it,
              and the market reaches a level when it feels like it.

              With no price it puts one just outside the pair's first band —
              the state hardest to draw, where the labels crowd.

              It was called `alert` while that was all it drew. It draws every
              card now, and a folder named after one of them was a lie.

  listen.rs   Opens Twelve Data's live price stream, asks for one symbol, and
              prints whatever comes down the line. Kept as the proof.

  README.txt  This file.


HOW watch/ SPENDS ITS REQUESTS

  IT COSTS NOTHING TO RUN. One request per pair per timeframe at startup to
  size the bands, and after that every price arrives on the socket for free.

  That is what killed the earlier design, where a candle was fetched on every
  close on every pair whether anything had happened or not.

  Two things keep the startup inside the limit of 8 requests a minute:

      - 7.5 seconds between requests, which is 8 a minute exactly
      - a timeframe a pair has no levels on is never asked about

  Four pairs across three timeframes would be twelve requests. Skipping the
  empty ones makes it seven.


WHY listen.rs EXISTS

  The whole design hangs on the websocket. Prices come down it for free, and
  they are what tell us when price reaches one of his levels.

  The free plan lists the websocket as 8 credits, 1 connection, marked TRIAL.
  Nobody knew whether it worked.

  It does. What it answered, on 14 August 2026:

      the line opens              101 Switching Protocols
      gold is allowed             status ok, XAU/USD in success, fails null
      prices flow                 15 in 19.5 seconds, about one a second
      what a price message is     symbol, timestamp, price. Nothing else

  Three things it also showed, which shaped what came after:

      - Several prices share the same timestamp. It is in whole seconds, so
        it cannot be used to put prices in order.

      - The price barely moves between messages. A touch test has to fire
        ONCE when price enters a band, not once per message, or one touch
        becomes twenty alerts. That is what nsc-core::levels::watch does.

      - Gold comes back as exchange COMMODITY, a blend of sources. Crypto
        comes from one exchange. That is why gold has five decimals and will
        never match his broker exactly.


THE SYMBOL IN listen.rs IS TEMPORARY

  It says BTC/USD, on purpose. Gold is shut at the weekend and a silent line
  looks exactly like a broken one; crypto trades all weekend.

  Change it back to XAU/USD when the market is open — Sunday 22:00 UTC.
