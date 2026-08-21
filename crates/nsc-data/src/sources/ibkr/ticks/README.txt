ticks/ — the live price line
============================


WHAT THIS FOLDER IS FOR

  Holding a price line open for every pair, and turning what comes down it
  into prices the bot understands.


THE FILES

  mod.rs         The front door.

  spread.rs      Spread -- the last bid, the last ask, and the middle.

  listening.rs   Subscribing to every pair, folding them into one channel,
                 and deciding what each tick means.

  tests.rs       Ten tests. The middle, and which notices matter.

  README.txt     This file.


IBKR NEVER SENDS A PRICE

  It sends a bid, and separately an ask.

      Twelve Data     {"price": "1.16413"}     one message, one number
      IBKR            Bid  1.16412             two messages, arriving at
                      Ask  1.16414             different moments

  So the middle is worked out here, and nothing comes back at all until both
  sides have arrived once. A middle worked out from a bid alone is not the
  middle of anything.

  IT MUST BE THE MIDDLE. The candles come back as MidPoint, so a live price
  taken from the bid would be measured against bands drawn on something else.


A MARKET THAT HAS NOT MOVED SAYS NOTHING

  Prices arrive several times a second and nearly all of them describe the
  same market as the last one. Only a middle that actually moved is passed on.

  It compares against the price immediately before, and nothing further back.
  Remembering every price ever seen would silence a market ticking between two
  numbers -- which is exactly what price does while it sits on one of his
  levels.


ONE SUBSCRIPTION PER PAIR, ONE LINE OUT

  Twelve Data took every symbol on a single socket. IBKR gives one connection
  per contract, so they are folded back into one channel here and the watcher
  keeps the loop it has always had.


THE FAILURE THIS FOLDER EXISTS TO CATCH

  IBKR DOES NOT FAIL A SUBSCRIPTION IT WILL NOT SERVE. It sends one notice
  down a line that stays open, and then never sends a price.

  Nothing arrives. Nothing errors. It is indistinguishable from a quiet
  market. So a notice becomes Heard::Refused and travels to the watcher.

  DELAYED PRICES ARE THE SAME TRAP WEARING A DISGUISE. An account without live
  forex data is served fifteen-minute-old prices instead of nothing at all.
  Dropped quietly the bot goes silent; acted on, it tells him price is at his
  level a quarter of an hour after it was. So it is said out loud, once, and
  the prices are ignored.

  MOST NOTICES ARE IBKR CLEARING ITS THROAT. "Market data farm connection is
  OK" arrives on every connection. Codes 2100-2200, and 1101/1102, are
  ignored -- passing those on would report a healthy feed as refused every
  time the bot started.
