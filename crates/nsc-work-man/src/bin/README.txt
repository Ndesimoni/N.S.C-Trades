bin/ — small programs that are not the bot
==========================================


WHAT THIS FOLDER IS FOR

  Anything in here becomes its own program, run with:

      cargo run -p nsc-work-man --bin <name>

  They are separate from the bot because they have a different shape. The bot
  does one job and exits. These hold a line open, or answer one question and
  stop.


THE FILES

  listen.rs   Opens Twelve Data's live price stream, asks for one symbol, and
              prints whatever comes down the line.

  README.txt  This file.


WHY listen.rs EXISTS

  The whole design hangs on the websocket. Prices come down it for free, and
  they are what tell us when price reaches one of his levels. Without it we
  would be back to asking for a candle every hour, on every pair, whether
  anything happened or not.

  The free plan lists the websocket as 8 credits, 1 connection, marked TRIAL.
  Nobody knew whether it worked.

  It does. What it answered, on 14 August 2026:

      the line opens              101 Switching Protocols
      gold is allowed             status ok, XAU/USD in success, fails null
      prices flow                 15 in 19.5 seconds, about one a second
      what a price message is     symbol, timestamp, price. Nothing else

  Three things it also showed, which matter later:

      - Several prices share the same timestamp. It is in whole seconds, so
        it cannot be used to put prices in order.

      - The price barely moves between messages. A touch test has to fire
        ONCE when price enters a band, not once per message, or one touch
        becomes twenty alerts.

      - Gold comes back as exchange COMMODITY, a blend of sources. Crypto
        comes from one exchange. That is why gold has five decimals and will
        never match his broker exactly.


THE SYMBOL IN IT IS TEMPORARY

  It says BTC/USD, on purpose. Gold is shut at the weekend and a silent line
  looks exactly like a broken one; crypto trades all weekend.

  Change it back to XAU/USD when the market is open — Sunday 22:00 UTC.
