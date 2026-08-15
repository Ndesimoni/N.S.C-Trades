retry/ — doing it again when that would help
============================================


WHAT THIS FOLDER IS FOR

  Running a job, and running it again if the trouble says another go is worth
  it.


WHY IT IS HERE AND NOT IN nsc-core

  BECAUSE IT SLEEPS.

  Waiting is doing. Knowing whether a failure is worth another go is knowing,
  and that lives in nsc-core::error as Answer and Knows.

  Nobody noticed the difference by reading. The compiler found it the moment
  the crates were split — nsc-core has no tokio, so nothing in it can wait for
  anything, and this would not build there.


THE FILES

  mod.rs      The front door.
  again.rs    keep_trying.
  tests.rs    Three tests.
  README.txt  This file.


IT GIVES UP TWO WAYS

  - the moment the trouble says GIVE UP, however many goes are left
  - after the goes it was given, even when the trouble says otherwise

  KEEP TRYING IS NOT THE SAME AS FOREVER.

  The wait doubles each time. Their end being busy is rarely fixed by asking
  again immediately.


THE TESTS DO NOT SLEEP

  The real waits are seconds long, which is right in the field and wrong in a
  test. These use a pretend trouble that clears in a millisecond.

  They took nine seconds before that. A test that sleeps is a test people stop
  running.
