swings/ — the swing finder, on a real chart
===========================================


WHAT THIS IS FOR

      cargo run -p nsc-work-man --bin swings -- XAU/USD 4h 300
      cargo run -p nsc-work-man --bin swings -- EUR/USD 1d 200

  Runs nsc-ta::swings over live IBKR history and prints every swing it proved,
  with the column that matters: KNOWN AFTER.


WHY THAT COLUMN IS THE POINT

  A swing is not knowable on the candle it sits on. You need candles
  afterwards to prove a peak was a peak -- and how many is NOT FIXED.

  Measured on real gold, 20 August 2026:

      4-hour    1 candle at best, 5 at worst
      daily     1 candle at best, 21 AT WORST

  Twenty-one candles is three weeks. The old rule said three candles, always,
  and that was a stand-in for this.


IT WILL OFTEN FIND NOTHING RECENT, AND THAT IS CORRECT

  The run in progress has not proved itself yet. On the 4-hour run above it
  found no swing after 4 August, because the move into 4,541 had not given
  back half, and had not given back the shallower share and then resumed.

  It is plain on the chart and the bot still cannot use it. By his own rule it
  has not proved itself.


THE NEWEST CANDLE IS DROPPED

  It is still forming, and a swing proved by a candle that has not closed is a
  swing that can un-prove itself.
