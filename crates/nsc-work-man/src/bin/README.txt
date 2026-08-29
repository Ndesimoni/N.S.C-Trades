bin/ — small programs that are not the bot
==========================================


WHAT THIS FOLDER IS FOR

  Anything in here becomes its own program, run with:

      cargo run -p nsc-work-man --bin <name>

  They are separate from the bot because they have a different shape. THE BOT
  IS `cargo run -p nsc-work-man` and it is the price watcher — that lives in
  src/watch/, not here, because it is the product rather than a tool.

  These answer one question and stop.


THE FILES

  history/    PULL YEARS OF CANDLES ONCE and keep them on disk, so a detector
              can be worked on without asking IBKR the same question fifty
              times. Writes data/history/<pair>-<timeframe>.csv.

  scan/       LOOK AT EVERY PAIR on the 1-hour and the 4-hour and say what is
              at his levels right now.

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
                  --bin cards -- news                  what is coming up
                  --bin cards -- news busy             several at once
                  --bin cards -- heartbeat             the quiet-day card
                  --bin cards -- armed                 a level going live
                  --bin cards -- trouble down|back|stopped

              The design loop. Changing how a card looks means looking at it,
              and the market reaches a level when it feels like it.

              With no price it puts one just outside the pair's first band —
              the state hardest to draw, where the labels crowd.

              It was called `alert` while that was all it drew. It draws every
              card now, and a folder named after one of them was a lie.

  listen.rs   Opens IBKR's live price line for one pair and prints every tick
              whole. The window onto what IBKR actually sends.

                  cargo run -p nsc-work-man --bin listen
                  cargo run -p nsc-work-man --bin listen -- XAU/USD


  candles/    WHERE IBKR STARTS ITS DAY. See its own README.

  after/      WHAT PRICE ACTUALLY DID NEXT after each pattern, against what an
              ordinary candle did over the same stretch. The base rate is the
              whole point, and the noise column is the second point. Its own
              README.

  README.txt  This file.


WHY listen.rs EXISTS

  The whole design hangs on prices arriving. They cost nothing, and they are
  what tell us when price reaches one of his levels. Everything else — the
  candle at a zone — only happens once a price has got there.

  IT IS ALSO THE ONLY WAY TO SEE IBKR REFUSE. IBKR does not fail a
  subscription it will not serve: it sends one notice down a line that stays
  open, and then never sends a price. Nothing errors. On the bot that is
  turned into a message; here you see it raw.

  WHAT TO LOOK FOR:

      Bid and Ask arriving        the feed works
      a Notice and no prices      IBKR is refusing that pair — almost always
                                  a market data subscription the account
                                  does not have
      DelayedBid / DelayedAsk     no LIVE data for that pair. Fifteen minutes
                                  behind, and the bot refuses to trade off it
      nothing at all              TWS is not logged in, or the market is shut


GOLD IS THE ONE TO CHECK

  XAU/USD goes to IBKR as a COMMODITY, not a forex pair, and spot metals are a
  separate market data subscription from spot forex. A paper account often has
  neither.

      cargo run -p nsc-work-man --bin listen -- XAU/USD

  A silent line looks exactly like a shut market, so check it while the market
  is open — it opens Sunday 17:00 New York.
