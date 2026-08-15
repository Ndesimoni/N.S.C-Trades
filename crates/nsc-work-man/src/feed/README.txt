feed/ — asking Twelve Data for candles
======================================


WHAT THIS FOLDER IS FOR

  One request, one answer. The live price stream is a separate thing.

  THE CANDLE IS NEVER COMPUTED. It comes from the feed finished, exactly as it
  appears on his chart. Building one out of smaller candles or out of ticks
  would produce something close to the broker's, never the same — and then
  nobody could say which was right. That assumption is what got the previous
  version of this project cleared out.


THE FILES

  mod.rs      The front door.
  ask.rs      The asking.
  error.rs    What can go wrong, and whether another go would help.
  tests.rs    Seven tests, all on that second question.
  README.txt  This file.


THE TRAP, WHICH CAUGHT US ONCE

  TWELVE DATA REFUSES WITH A PERFECTLY ORDINARY REPLY.

  A 200, and {"code": 401, "status": "error"} in the body. So a reply that
  parses is not a reply that worked, and the code has to be read out of the
  BODY rather than off the response.


WHICH FAILURES ARE WORTH ANOTHER GO

    try again                     give up
      the line dropped              no API key
      their end fell over (5xx)     the key is wrong (401)
      too many requests (429)       the pair is not on the plan (404)
      the reply was not candles

  429 waits a minute rather than three seconds. They have told us to slow
  down, so hammering is both rude and pointless.

  A wrong key stopping on the FIRST go is the point of all of this. Retry it
  and it looks exactly like a dead connection, and you spend an evening
  checking your wifi.
