tests/ — checking the levels
============================


THE FILES

  mod.rs       The front door.

  bands.rs     Does the band we build land where the one he drew landed?

  pips.rs      One pip, worked out from how the pair is quoted. Gold to two
               decimals is 0.10; the euro to five is 0.0001.

  saving.rs    Levels arriving from his phone, and going back off again.

  watching.rs  Firing once per touch, not once per price — and the two
               different distances for arriving and leaving.

  support.rs   The scratch folder and the price helper they share.

  README.txt   This file.


THE ONE THAT MATTERS

  bands.rs holds his own measurements, taken off his TradingView chart:

      4132.020 - 4055.913   a weekly band he drew at 4094
      3383.480 - 3303.553   another, drawn months earlier at 3344
      3000.463 - 2968.181   a daily one at 2984

  The tests build bands from the LINE PRICE ALONE and check they come out
  within a point or two of those.

  That is the whole point. Everything in this project is measured against his
  levels, so a change that makes our band stop matching his hand has to fail
  loudly rather than quietly draw something slightly different.


THE ONE THAT IS EASIEST TO GET WRONG

  watching.rs::leaving_takes_more_than_it_took_to_arrive.

  ARRIVING and LEAVING are measured differently — a pip to arrive, a tenth of
  the band to leave — and it would be very natural to tidy that into one
  number. That test fails the moment anyone does, which was checked by making
  the change and watching it go red.


WHY saving.rs WRITES REAL FILES

  Saving a level is file work — appending without losing what is there,
  keeping the comments, and a price surviving the trip through TOML exactly.
  None of that can be checked without a real file.

  Each test gets its own scratch folder under the system temp directory, so
  they cannot tread on each other however they are run.
