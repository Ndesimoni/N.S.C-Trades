news/ — downloading the economic calendar
=========================================


WHAT THIS FOLDER IS FOR

  Getting the week's file and turning it into events. That is all.

  THE JUDGEMENT IS NOT HERE. Whether an event is due, and what shares a card,
  is nsc-core::news. This folder gives back every row in the file, of every
  rating, and lets that crate decide.

  Keeping them apart is what lets the deciding be tested without a network.


THE FILES

  mod.rs      The front door.
  error.rs    What went wrong, and whether another go would help.
  parse.rs    The feed's own shape, turned into nsc-core events.
  feed.rs     The asking, and the refusal that arrives looking fine.
  tests.rs    Eleven tests. None of them touch the network.
  README.txt  This file.


WHY THIS IS NOT IBKR

  IBKR's API carries news HEADLINES -- six calls, all of them articles from a
  provider. It has no macro calendar at all. No rate decisions, no payrolls,
  nothing scheduled with a time on it.

  Checked in ibapi 2.12: news_providers, news_bulletins, historical_news,
  news_article, contract_news, broad_tape_news. There is a Wall Street Horizon
  calendar too, and it is corporate events -- earnings and dividends, for
  stocks.

  So this is a second source, and the only thing in this crate that does not
  come from the broker.


THE REFUSAL THAT ARRIVES LOOKING LIKE A SUCCESS

  Two downloads every five minutes is the limit, shared across all four
  formats of the same file.

  GO OVER AND IT DOES NOT RETURN AN ERROR. It returns an HTML page saying
  "Request Denied", under a perfectly normal 200.

  Handed straight to the JSON parser that comes back as a parse failure, which
  reads as "the feed changed shape" -- and that answer is GiveUp. One busy
  afternoon would have retired the news watcher for good.

  So feed.rs looks at the first character before parsing. A JSON array starts
  with [, a web page starts with <. Deliberately the weakest test that tells
  them apart, because anything cleverer is more to be wrong about.

  THIS IS THE THIRD TIME. Twelve Data refused with a polite {"code": 401}.
  Telegram refuses with a polite ok: false. A reply that parses is not a reply
  that worked.


ONE BAD ROW DOES NOT COST THE WEEK

  A row whose time will not parse is counted in `unreadable` and left out.

  Counted rather than dropped quietly: one bad row must not lose the whole
  calendar, but a row silently vanishing is how a feed change goes unnoticed
  for a month. The caller is told how many and says so out loud.


EVERYTHING IS UTC AFTER parse.rs

  They stamp events with a New York offset -- 2026-08-25T10:00:00-04:00. It is
  converted once, on the way in. Kept as written it would be an hour out for
  half the year and nothing would error, which is exactly the trap the daily
  candle boundary set.
