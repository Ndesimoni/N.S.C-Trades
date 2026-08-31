tests/ — checking the levels
============================


THE FILES

  mod.rs       The front door.

  bands.rs     Does the band we build land where the one he drew landed?

  pips.rs      One pip, worked out from how the pair is quoted — gold to two
               decimals is 0.10, the euro to five is 0.0001. Also a pair
               asking for its own approach distance instead of the shared one.

  closing.rs   Where a finished candle ENDED at a zone — above, below,
               inside, or nowhere near.

  acting.rs    What KIND of thing it did there — kissed it, pushed back, cut
               through. A wick that grazed the edge and a candle that drove a
               third of the way in both "closed above".

  saving.rs    Levels arriving from his phone.

  removing.rs  Undo, which reaches what the last message added, and taking
               one level off by its price, which reaches any of them.

  stopping.rs  Setting a whole pair aside, and putting it back.

  watching.rs  Firing once per touch, not once per price — and the two
               different distances for arriving and leaving.

  breaking.rs  What a CLOSE at a level is worth. A candle that broke through
               the way price was travelling is news; one thrown back where it
               came from is not. Split out of watching.rs on 31 August 2026,
               when that file reached 311 lines of code against a 170 limit.

  support.rs   The scratch folder, the price helper, the two bands the tests
               are written against — his gold weekly and his AUD/USD daily —
               and a candle built from just an open and a close.

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


THE ONE THAT WOULD HAVE BEEN MISSED

  pips.rs::a_pair_file_can_carry_its_own_number.

  Every other test there builds a Pair in memory, and all of them would pass
  with the setting unreadable from an actual file. TYPING IT INTO THE FILE IS
  THE ONLY WAY HE WILL EVER SET IT, so the trip through TOML is the part that
  matters.


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
