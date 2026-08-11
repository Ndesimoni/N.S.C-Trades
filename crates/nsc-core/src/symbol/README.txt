symbol/ — what an instrument is
===============================


WHAT THIS FOLDER IS FOR

  Holding everything the system needs to know about one thing you trade.

  EURUSD. Gold. US30. Oil.

  Mostly this is small facts that other parts of the code keep needing:
  how big a pip is, how many decimal places to show, how wide a spread is too
  wide, and which currencies are involved.


THE FILES

  mod.rs          The front door.
                  Says what the folder does, and lets the rest of the program
                  see three things: AssetClass, Currency and Symbol.

  class.rs        What KIND of thing it is: forex, metal, index or energy.
                  Matches the "class" field in config/symbols.toml.
                  This is not just a label. See below.

  currency.rs     A three-letter code. USD, EUR, JPY.
                  Checked when you create it, not trusted.

  instrument.rs   The instrument itself. Name, class, pip size, decimal
                  places, spread limit, and the two currencies.

  tests.rs        Eight tests.


HOW THEY FIT TOGETHER

      class.rs    ─┐
                   ├─►  instrument.rs     a Symbol holds one of each
      currency.rs ─┘

      instrument.rs ─►  price/            for pip conversions and rounding

      mod.rs        ─►  lets the outside world see all three


WHY "class" IS NOT JUST A LABEL

  These instruments genuinely behave differently.

  Equity indices settle an hour earlier than the forex day.
  Indices and oil stop trading at night. Forex does not.
  When a market stops and restarts, you get a gap — and a gap is not a
  candle, so it must not be read as one.

  Code that needs to treat those differently has to be able to ask what kind
  of thing this is. That is what class.rs is for.


WHY A CURRENCY IS A TYPE AND NOT JUST TEXT

  Because of how the mistake shows up.

  Type "USDD" into a plain piece of text and nothing complains. The news
  filter just never matches any event. Your news blackout quietly does
  nothing, forever.

  No error. No crash. You would only notice months later, when you saw the
  bot take a trade straight through a jobs number.

  Checking it here means the typo is caught the moment the program starts, on
  the line where it was written.


NOT EVERYTHING HAS TWO CURRENCIES

  EURUSD has two: EUR and USD.
  US30 has one: it is priced in USD, but there is no "base".
  Gold's base is a metal, not a currency.

  So both currencies are optional.

  This matters for the news filter. Its question is "does this US
  announcement affect this instrument?" For EURUSD the answer comes from the
  currencies. For US30 there is nothing to check, so it answers no.

  That is deliberate. Indices need their own news rule. Better to leave the
  gap visible than to invent a fake base currency and quietly get it wrong.


BAD SETTINGS ARE REFUSED AT STARTUP

  A pip size of zero would make every stop distance a divide by zero.

  So Symbol::new refuses it, and names the instrument in the error.

  The alternative is failing on some candle six hours into a backtest, where
  you would have no idea which instrument caused it.


THE SMALL HELPERS

  to_pips              turns a distance into pips using THIS instrument's
                       pip size, so nobody has to remember to look it up

  spread_is_acceptable is the live spread tight enough to bother trading

  format_price         rounds for showing a human — never for comparing.
                       Rounding before you compare a price to a level is how
                       a level check starts lying to you.
