nsc-data — where prices come from
=================================


WHAT THIS CRATE IS FOR

  Everything that talks to a broker. Candles and live prices both.

  NOTHING OUTSIDE THIS CRATE KNOWS WHICH BROKER YOU USE. The bot asks for "the
  last 60 weekly candles for EUR/USD" and gets them. No address, no key, no
  wire format lives anywhere else.


THE FILES

  lib.rs      The front door — two modules and nothing else.

  source/     WHAT THE BOT ASKS FOR. The trait every feed answers, what a
              timeframe is, and what a live price is.

  sources/    WHO ANSWERS. One folder per broker.

  README.txt  This file.


source/ IS THE QUESTION, sources/ IS THE ANSWER

  The one-letter difference is deliberate and it is easy to misread, so it is
  worth saying plainly:

      source/     the shape of the question. No broker names in it.
      sources/    the brokers. Every company name lives here.


THE FEED IS IBKR, AND ONLY IBKR

  Since 20 August 2026. Twelve Data was the feed before that and is gone --
  candles and the live price line both come from Interactive Brokers now.

  WHAT THAT COSTS:

    - TWS OR IB GATEWAY HAS TO BE RUNNING and logged in. There is no feed at
      all without it, and there is no fallback.

    - Gold needs its own market data subscription. Spot metals are not spot
      forex at IBKR.

    - IBKR sends a bid and an ask, never a price. The middle is worked out in
      sources/ibkr/ticks/.


IT IS A LIBRARY, SO IT USES thiserror

  The caller has to be able to tell a missing .env line from a dropped
  connection, because one is worth retrying forever and the other never is.

  Every error answers one question: IS IT WORTH TRYING AGAIN? See
  nsc-core::error::Knows.
