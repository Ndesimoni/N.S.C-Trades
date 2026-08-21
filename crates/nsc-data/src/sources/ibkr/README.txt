ibkr/ — Interactive Brokers
===========================


WHAT THIS FOLDER IS FOR

  The whole feed. Candles, live prices, and the account.

  THIS FEED NEEDS A PROGRAM RUNNING. Unlike a web API, nothing here works
  unless TWS or IB Gateway is logged in and reachable at IBKR_HOST:IBKR_PORT.
  That is the cost of using it, and it is paid at deploy time.


THE FILES

  mod.rs       The front door.

  connect.rs   Opening the line, the account summary, the live price line,
               and a raw tick window for looking at what IBKR actually sends.

  contract.rs  "EUR/USD" into something IBKR will accept. GOLD IS NOT A FOREX
               PAIR here -- it is a commodity, routed differently, needing a
               different market data subscription.

  candles.rs   Asking for candles, and turning them into ours. This is where
               an IBKR timestamp becomes UTC and an f64 stops being one.

  error.rs     What can go wrong, and whether another go would help.

  ticks/       The live price line.

  tests.rs     Empty for now.

  README.txt   This file.


THE TWO THINGS THAT ARE WRONG WITH AN IBKR CANDLE

  ITS STAMP IS IN WHATEVER TIMEZONE TWS WAS LOGGED IN WITH. His is Dubai.
  Left alone every candle would be four hours out and NOTHING WOULD ERROR.

  Fixed by going through unix_timestamp -- the same number in Dubai as in
  London -- so nothing has to know what TWS was set to.

  ITS PRICES ARE f64. This project never keeps a price that way.


THE TIMEZONE ALIAS IS NOT OPTIONAL

  TWS reports the machine's timezone with a Windows name -- "Gulf Standard
  Time" -- and the library only knows the IANA ones. Without the alias,
  connecting fails outright with an error that says nothing about timezones.

  It is registered in connect.rs before anything else happens.


ASKING FOR THE RIGHT THING

  MidPoint, not TRADES. Spot forex has no central exchange to trade on, so
  asking for trades is refused.

  Extended, not Regular. Regular hours would cut the day down to an exchange
  session that spot forex does not have, and the candles would stop matching
  his chart.
