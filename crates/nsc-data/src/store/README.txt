store — the record
==================


WHAT THIS FOLDER IS FOR

  What the bot DECIDED, and what the market did about it.

  IT IS A RECORD, NOT A CACHE. What price is doing right now belongs in
  memory. What was decided cannot be recreated afterwards, so it lives here.

  Three questions the bot could not answer before it existed:

    "Why did nothing fire last week?"   nothing was written down, so a quiet
                                        week and a broken bot looked identical
    "Does the level actually save it?"  rung 3 is a test with nothing to
                                        measure it against
    "What happened after this setup?"   signals vanished the moment they sent


THE FILES

  mod.rs      The front door, and what the outside world can see.
  error.rs    What can go wrong, and the one question each answers: try
              again, or give up?
  pool.rs     Opening it, and running the migrations on the way in.
  candles.rs  The history everything else is measured against.
  tests.rs    Four that need Postgres, and one that does not.
  README.txt  This file.

  The tables themselves are in migrations/ at the top of the project, and
  the reasoning behind every column is docs/worksheets/database.md.


SQL LIVES HERE AND NOWHERE ELSE

  Every query hands back an nsc-core type, never a raw row. A table can change
  shape and the change stops inside this folder.

  nsc-core AND nsc-strategy NEVER TOUCH THIS. Neither has sqlx in its
  manifest, and it is the manifest that enforces it rather than a rule
  somebody remembers. A rule that needs a row gets handed the row -- which is
  the whole of what lets the backtester and the live bot run the same
  analysis and get the same answer.


RUNTIME QUERIES, NOT THE query! MACRO

  sqlx can check SQL against a live database at COMPILE time. That would make
  `cargo check` need Postgres running, and the whole workspace would stop
  building whenever the container was down.

  THE COST IS THAT A TYPO IN SQL IS FOUND ON THE FIRST CALL rather than by the
  compiler. So every query gets a test that runs it, and those tests are the
  first call.


WHY THE DATABASE TESTS ARE #[ignore]

  The ordinary suite has to pass on a machine with no container.

    docker compose up -d
    cargo test -p nsc-data -- --ignored

  AND THEY MUST NOT QUIETLY PASS WHEN THERE IS NO DATABASE. A test that skips
  itself and reports green pins nothing at all. #[ignore] says "not run" out
  loud; skipping inside the body would say "passed".

  THEY WRITE TO A DATABASE OF THEIR OWN -- nsc_trades_test, made on demand on
  the same server. The first version wrote to the real one and only cleared on
  the way IN, so TST/ROUNDTRIP and friends piled up in the record beside his
  candles. THE RECORD IS MEANT TO BE THE TRUTH; a fake pair in it gets counted
  by something eventually.

  EVERY TEST ALSO USES ITS OWN SYMBOL. They shared one at first and each
  cleared it on the way in, so in parallel they wiped each other -- green
  alone and red together, which is the worst way round for a test to fail.


NEVER A FLOAT

  NUMERIC(18,8) everywhere a price appears. 0.1 + 0.2 is not 0.3 in floating
  point, and a level at 4520.00 stored as 4519.9999998 answers "did price
  touch it" with NO while his eye says yes.


EVERYTHING UTC

  TIMESTAMPTZ, and TZ=UTC on the container itself. A server on local time
  shifts the daily candle, which shifts every band, which changes every
  signal -- and it looks like a strategy problem rather than a clock one.
