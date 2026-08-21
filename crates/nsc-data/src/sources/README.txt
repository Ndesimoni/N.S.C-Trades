sources/ — who answers
======================


WHAT THIS FOLDER IS FOR

  One folder per broker. EVERYTHING THAT KNOWS A COMPANY'S NAME LIVES HERE.

  source/ next door is what the bot asks for. This is who answers it.


THE FILES

  mod.rs      The front door.

  ibkr/       Interactive Brokers. The only feed, as of 20 August 2026.

  README.txt  This file.


TWELVE DATA IS GONE

  Removed 20 August 2026. It was the feed from 14 August, and everything --
  candles and the live price line both -- now comes from IBKR.

  WHAT THAT COSTS, WRITTEN DOWN SO IT IS NOT A SURPRISE:

    - IBKR needs a program running. TWS or IB Gateway has to be logged in and
      reachable, or there is no feed at all. Twelve Data was a web request and
      needed nothing running.

    - Gold is a separate market data subscription. Spot metals are not spot
      forex at IBKR, and an account can easily have one and not the other.

  There is no fallback. If IBKR is down, the bot is down.
