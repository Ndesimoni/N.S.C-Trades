fib/ — the Fibonacci on a real chart
====================================


WHAT THIS IS FOR

      cargo run -p nsc-work-man --bin fib -- XAU/USD 4h
      cargo run -p nsc-work-man --bin fib -- XAU/USD 1h 200

  Draws the four levels on live IBKR history and says where price sits in
  them: barely paused, coming back, in the golden zone, deeper, past the stop,
  undone, still going.


WHICH MOVE IT MEASURES

  THE RATIOS ARE THE EASY PART. WHICH MOVE YOU MEASURE IS THE WHOLE GAME.

  THE MOVE SINCE THE LAST CONFIRMED SWING. The near end is a point the chart
  PROVED -- price left it and gave back enough to prove it. The far end is
  where price has got to since, which has NOT proved itself and may never.

  That is the move in progress, and it is the one you would draw by hand.

  It falls back to the window's high and low when there are not two swings
  yet, AND SAYS SO, because those two answers can be wildly different.


  TWO WORSE VERSIONS, KEPT HERE BECAUSE THEY EXPLAIN THE THIRD

  ANCHORED ON A WINDOW'S HIGH AND LOW, gold read like this on 21 August:

      hourly      4,311 -> 4,541 up      230 points   barely paused
      four-hour   3,996 -> 4,541 up      545 points   barely paused
      daily       5,239 -> 3,942 DOWN  1,297 points   coming back
      weekly      2,287 -> 5,608 up    3,322 points   barely paused

  The daily measuring the OPPOSITE DIRECTION from everything else, because the
  window happened to start above the year's high.

  ANCHORED ON THE LAST TWO CONFIRMED SWINGS, the four-hour gave a 41-POINT
  MOVE FROM THREE WEEKS AGO while price had run 500 points since. Both swings
  are proved, so the second one is necessarily old -- nothing confirms until
  price has given back half of it. A perfectly correct answer to a question
  nobody asked.

  ANCHORED ON THE MOVE SINCE THE LAST SWING, the same moment reads:

      30-minute   4,605 -> 4,564 down     41 points   in the golden zone
      hourly      4,325 -> 4,605 up      280 points   barely paused
      four-hour   4,065 -> 4,605 up      539 points   barely paused
      daily       3,942 -> 4,605 up      663 points   barely paused
      weekly      3,942 -> 4,605 up      663 points   barely paused

  FOUR OF THE FIVE SHARE A TOP -- 4,604.73, found independently on four
  charts, because it is the high.

  The 30-minute is the odd one and correctly so: it is the only chart short
  enough to have CONFIRMED a swing at that high, so its move is the pullback
  since.


PRICES ARE ROUNDED TO THE PAIR'S OWN PRECISION

  A fib level is a share of a move, so the arithmetic runs long. 4332.89674 on
  an instrument quoted to two decimals is five digits of false confidence, and
  it looks authoritative in a way it has not earned.
