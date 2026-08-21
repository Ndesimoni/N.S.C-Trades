source/ — what the bot asks for, whoever is answering
=====================================================


WHAT THIS FOLDER IS FOR

  The door every feed comes through.

  NOTHING ABOVE THIS FOLDER KNOWS WHICH BROKER YOU USE. The watcher asks for
  "the last 60 weekly candles for EUR/USD" and gets them. Which company
  answered is decided once, at startup, in sources/.

  That is not about keeping a spare feed. It is about where broker details are
  allowed to live. A websocket address sat inside the price watcher for
  months, and that one line is why changing feed was a job rather than an edit.


THE FILES

  mod.rs       The front door.

  candles.rs   MarketDataSource — the one thing every feed must do.

  interval.rs  Interval — how long one candle lasts. A TYPE, not a string.

  price.rs     Price — one live price. Heard — what comes down the price
               line: a price, a refusal, or a line that ended. And Prices,
               the open line itself.

  tests.rs     Four tests, all on Interval.

  README.txt   This file.


WHY INTERVAL IS A TYPE AND NOT A STRING

  These used to be written out by hand at every call site -- "1week", "4h",
  "1day" -- in whatever spelling that week's provider wanted.

  A typo in one of those does not fail to compile. It fails at the feed, at
  runtime, on ONE timeframe only, and that pair quietly stops reporting while
  everything else carries on.


WHY A PRICE IS THE MIDDLE OF THE SPREAD

  The candles come back as MidPoint. So a live price taken from the bid would
  be measured against bands drawn on something else.

  On the euro that is a fifth of a pip and nobody notices. On gold the spread
  is around 30 cents, which is most of a band edge: the alert says price
  touched his level and the candle card then says it never got there.


WHY PRICES IS A TYPE AND NOT A BARE CHANNEL

  BECAUSE DROPPING IT HAS TO STOP THE SUBSCRIPTIONS.

  Each pair is carried by its own task sitting on the next tick. A task only
  notices the line has been put away when it next tries to send -- so a QUIET
  pair would sit there forever, holding an IBKR market data line nobody reads.

  The watcher reopens the line every time he sends a level. Twenty levels over
  a weekend is twenty abandoned subscriptions per pair, against an IBKR limit
  counted in LINES rather than in pairs. It would stop serving new ones and
  nothing would say why.


WHY HEARD HAS A "REFUSED" IN IT

  BECAUSE IBKR DOES NOT FAIL A SUBSCRIPTION IT WILL NOT SERVE.

  It sends one notice down a line that stays open, and then never sends a
  price. Without somewhere for that notice to go, it is indistinguishable from
  a quiet market -- nothing arrives, nothing errors, and the bot looks like it
  is working.
