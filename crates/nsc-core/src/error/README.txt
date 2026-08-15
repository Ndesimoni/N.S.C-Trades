error/ — what went wrong, and what to do about it
=================================================


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

  answer.rs     Answer — try again after this long, or give up. And Knows,
                which every trouble in the project implements.

  tests.rs      Three tests, on the question itself rather than on any
                particular answer to it.

  README.txt    This file.


THE TROUBLES THEMSELVES ARE NOT IN HERE

  EACH ONE LIVES BESIDE THE THING THAT PRODUCES IT.

      candle/error.rs           CandleError
      levels/error.rs           LevelError
      ../nsc-work-man/feed/     FeedError
      ../nsc-work-man/telegram/ SendError
      ../nsc-work-man/card/     CardError

  They were gathered here first, and that was wrong in a way worth
  remembering: THIS CRATE HAD AN ERROR TYPE THAT MENTIONED TELEGRAM. A crate
  that cannot reach Telegram has no business knowing it exists.

  What is left here is only the shared question, which everything answers.

  The programs in bin/ use anyhow — something went wrong, here is the trail,
  stop. That is right for a program with a person watching it. A library needs
  a decision.


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
