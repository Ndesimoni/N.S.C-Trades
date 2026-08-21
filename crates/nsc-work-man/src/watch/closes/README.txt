RUNG 2 -- WHAT A CANDLE DID AT A ZONE
=====================================

Price arriving at a level says nothing. It may cut straight through. The
CLOSE is what says whether it was a rejection: a candle at 4,120 that closes
with a long wick back down is one thing, the same candle closing at its high
is the opposite. You only know which once it has finished.

Only pairs with price actually at a zone are ever fetched. A quiet week costs
nothing.


THE FILES

  mod.rs      The front door.

  said.rs     What one report was about -- pair, interval, kind, zone.
              This is the key everything is remembered by.

  due.rs      When a pair's next candle is worth asking about. See IT ASKS
              WHEN A CANDLE IS DUE below.

  look.rs     The ten-minute check. Works out which candle has finished and
              which is far enough into itself to be worth a look.

  report.rs   Saying what the candle did, one zone at a time.

  fetch.rs    Asking the feed.

  tests.rs    Three tests, all on the key. Nothing here reaches the feed.


IT NEVER WORKS OUT WHEN A CANDLE CLOSES

  The feed's own stamp says whether a candle is finished. Working the
  boundaries out here would mean knowing where the feed puts its 4-hour
  candles, which nobody has measured. Guessing wrong reports a candle that has
  not happened -- the mistake that makes results look better rather than
  broken.


IT ASKS WHEN A CANDLE IS DUE, NOT ON A TIMER

  It used to ask every ten minutes. A 4-hour candle closes six times a day, so
  about 140 of every 144 asks found nothing new -- 288 requests a day per pair
  to learn something that happens thirty times.

  Now it reads the stamp the feed already handed back. Told a candle is
  stamped 12:00 on the 4-hour, the feed has said where its own boundary is:
  the twenty-minute look falls at 13:20 and the close at 16:00. It waits for
  those.

  THIS IS NOT THE SAME AS WORKING OUT A BOUNDARY. It is reading one the feed
  stated, and the returned stamp is still what decides whether a candle has
  finished. Nothing is assumed.

  288 asks a day per pair down to 60. And a close report is no longer up to
  ten minutes late -- it lands when the candle does.

  There is a floor of one minute between asks about the same pair. Every
  moment worked out from a stale stamp is in the past, and without the floor
  "ask when the next is due" would mean "ask again immediately, forever".


THE KEY IS PER ZONE, NOT PER CANDLE

  It used to be remembered per candle, and that cost twice.

  A second zone coming live mid-hour never got that hour's candle at all.
  Price sits at 4,120, the 13:00 candle closes and is reported, price runs to
  4,135 at half past -- and the 13:00 candle is already marked done. He waited
  a full hour for news the bot was holding.

  And one card failing to send made every OTHER zone on that candle repeat on
  the next look, because they were remembered together or not at all.

  Now each zone is decided on its own. tests.rs pins it.


NOTHING SAID IS NOTHING REMEMBERED

  A zone with nothing worth saying is not marked. A forming candle is still
  moving -- nothing worth saying twenty minutes in may be worth saying at
  forty, and the stamp is the same one. Working it out again is arithmetic on
  candles already fetched, so it costs nothing.

  A card that fails to send is not marked either, so the next look tries it
  again. A close is the thing he is waiting for.


THE TWENTY-MINUTE LOOK

  A candle at one of his zones gets spoken about TWICE:

      about a third of the way in    what it has done SO FAR
      when it finishes               what it did

  The first is the only place in this project that reads a candle before it
  has finished. It is allowed for the same reason the price alert is — it is a
  heads-up and nothing more — and the card says so on its own face. IT MUST
  NEVER REACH A STRATEGY.

  NOT ON THE OPEN. Spot forex runs Sunday to Friday without a break, so a
  candle's open IS the last one's close. That message would repeat what
  arrived a minute earlier.

  ONE REQUEST SERVES BOTH. The reply already carries the candle that just
  finished and the one still running, so the look costs nothing on top.

  The two are remembered apart. Keyed together, the look would silence the
  close that follows it — the one that actually matters.

  `look_in_minutes` in config/when.toml is set for the 1-hour and scaled for
  the rest: twenty minutes into an hour is eighty into a 4-hour.

  IT LANDS ON THE MINUTE, NOT IN A TEN-MINUTE WINDOW. It used to be checked on
  a timer, so the look arrived somewhere between twenty and thirty minutes in.
  closes/due.rs now reads the stamp the feed already handed back and wakes at
  that candle's own look and close. Twenty minutes in means twenty.


IT NEVER WORKS OUT WHEN A CANDLE CLOSES

  closes/ asks for the newest candle when that pair's next one is due, and lets
  THE FEED'S OWN STAMP say whether it is one already reported.

  Working the boundaries out here would mean knowing where the feed puts its
  4-hour candles, which nobody has measured. Guessing wrong reports a candle
  that has not finished — and reading a candle early does not error, it makes
  results look better.

  A 4-HOUR CANDLE DOES NOT EXIST UNTIL ITS LAST HOUR HAS CLOSED. Three hourly
  closes can pass with the 4-hour still saying nothing; the fourth is when it
  speaks. Bar::finished_by is the single place that decides.
