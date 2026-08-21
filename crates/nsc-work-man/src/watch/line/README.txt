line/ — holding the price line open
===================================


WHAT THIS FOLDER IS FOR

  The loop that watches every price, and everything that can end it.

  It returns when the line closes, when the session goes quiet, or when he
  sends a level. WHAT HAPPENS NEXT IS NOT DECIDED HERE — run/ decides that.


THE FILES

  mod.rs        The front door.

  closed.rs     Closed — why the line stopped. Neither reason is a fault.

  refusals.rs   Refusals — which pairs IBKR has said no to, and the point at
                which that is worth stopping over.

  listen.rs     The loop. Prices on one side, the ten-minute housekeeping
                tick on the other.

  tests.rs      Five tests, all on refusals.rs.

  README.txt    This file.


ONE SUBSCRIPTION PER PAIR, ONE LOOP

  Twelve Data carried every symbol on a single socket. IBKR gives one
  connection per contract.

  nsc-data folds them back into one channel, so this loop is unchanged by the
  difference. That fold is the whole reason changing feed did not mean
  rewriting the watcher.


TWO THINGS COME OUT OF THE SELECT

  A price          -> fed into `arrive`, and an alert if it reached a zone
  The ten-minute   -> the heartbeat, the calendar, the levels-file check,
  tick                and rung 2 asking what a candle did

  THE PRICE IS RECORDED BEFORE THE GREETING RUNS, and the order matters. The
  greeting reports which zones price is RESTING IN, and nothing is resting
  anywhere until a price has been fed in. Run the other way round and the very
  first price of a session finds nothing, sends nothing, and marks the session
  greeted -- so the report of where price already stood never comes at all.


WHY REFUSALS IS A TYPE AND NOT A COUNTER

  IBKR DOES NOT FAIL A SUBSCRIPTION IT WILL NOT SERVE. It sends one notice
  down a line that stays open, and then never sends a price for that pair.

  So one refused pair is silent in exactly the way a quiet pair is silent.
  This is what notices its own silence.

  Three things had to be got right, and each was wrong once:

    - NOUGHT REFUSED OUT OF NOUGHT ASKED IS NOT A TOTAL REFUSAL. Removing his
      last pair left the watcher asking about nothing, 0 == 0 came out true,
      and it reported the price line as broken every thirty seconds over a bot
      doing exactly what it had been told.

    - THE SAME PAIR COMPLAINING TWICE IS STILL ONE PAIR. IBKR repeats a notice
      whenever it likes. Counted twice, gold alone would look like every pair
      refusing.

    - SOME REFUSED IS NOT ALL REFUSED. It is said out loud and the rest are
      watched, because one refused pair looks exactly like one quiet pair.


A LINE THAT ENDED LEAVES BY THE ERROR PATH

  Not as a clean finish. That way the five-minute rule in trouble.rs decides
  whether he hears about it -- quiet about hiccups, loud about outages.

  A clean Closed means the session ended or he sent a level. Nothing wrong.


THE GREETING IS ASKED AFTER THE PRICE, NOT BEFORE
=================================================

  line/listen.rs feeds each price into `arrive` and THEN asks the greeting whether
  it has anything to say. The order is the behaviour.

  The greeting reports which zones price is RESTING IN. Nothing is resting
  anywhere until a price has been fed in -- a fresh Watch has every band down
  as Away and no last price at all.

  Asked first, on the very first price of a session, it found nothing, sent
  nothing, and marked the session as reported. The report of where price
  already stood -- the whole reason it waits for the opening hours to pass --
  never came, for the entire session.

  It is guarded twice on purpose. The order here, and a check in
  resumed/awake.rs that skips any pair no price has arrived for. Staying true
  should not depend on two lines staying in one order.
