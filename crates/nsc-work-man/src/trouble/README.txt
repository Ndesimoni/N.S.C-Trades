trouble/ — try again, or give up?
=================================


WHAT THIS FOLDER IS FOR

  Every failure in this project answers ONE question:

      IS IT WORTH TRYING AGAIN?

  That is the whole reason the library has named troubles instead of one
  catch-all error.


WHY IT MATTERS MORE THAN IT SOUNDS

  A bot that cannot tell a bad API key from a dropped line does one of two
  things, and both are bad:

    - retries the bad key forever, and IT LOOKS EXACTLY LIKE A DEAD
      CONNECTION. You spend an evening checking your wifi.

    - or dies on a hiccup that would have cleared in three seconds, and you
      find out on Monday that it stopped on Friday.

  It is a rule in CLAUDE.md and it was ignored until now. Everything was
  anyhow, which loses the distinction entirely.


THE FILES

  mod.rs        The front door.

  answer.rs     Answer — try again after this long, or give up. And
                keep_trying, which does a job and respects the answer.

  tests.rs      Twelve tests. The first six are the distinction itself.

  README.txt    This file.


WHAT COUNTS AS WHICH

  TRY AGAIN                          GIVE UP
    the line dropped                   no API key
    their server fell over (5xx)       the key is wrong (401)
    too many requests (429)            the pair is not on the plan (404)
    the reply was not candles          the caption was too long
                                       the picture is not there

  429 waits a MINUTE rather than three seconds. They have told us to slow
  down, so hammering is both rude and pointless.


KEEP TRYING IS NOT THE SAME AS FOREVER

  keep_trying gives up two ways:

    - the moment the trouble says GIVE UP, however many goes are left
    - after the attempts it was given, even when the trouble says otherwise

  The wait doubles each time. Their end being busy is rarely fixed by asking
  again immediately.


ONE THING BOTH FEEDS DO THAT CAUGHT US TWICE

  THEY REFUSE POLITELY.

  Twelve Data answers 200 with {"code": 401, "status": "error"} in the body.
  Telegram answers 200 with ok: false.

  A reply that parses is not a reply that worked. Both are checked in the
  body, not on the response — and the tests cover both.


THE TESTS DO NOT SLEEP

  The real waits are seconds long, which is right in the field and wrong in a
  test. The retry tests use a pretend trouble that clears in a millisecond.

  They took nine seconds before that. A test that sleeps is a test people stop
  running.
