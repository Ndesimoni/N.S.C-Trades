fib/ — one move, and the shares of it
=====================================


WHAT THIS FOLDER IS FOR

  Saying what a Fibonacci retracement IS: a move, and the arithmetic for
  taking shares of it.

  Choosing WHICH move is not done here. That is nsc-ta::fibonacci.


IT HOLDS THE MOVE, NOT A LIST OF PRICES

  The prices are only ever a share of the move. Storing them instead would
  keep the answers and throw away the question.

  And the question is the thing worth arguing about. The same ratios drawn
  from a different pair of swings give completely different prices, so when a
  Fibonacci signal looks wrong the move it chose is nearly always the
  disagreement.

  Keeping the move means that argument can be settled by looking at a chart.


THE TWO QUESTIONS

  WHERE IS A GIVEN SHARE?   0.618 of this move is what price.

  HOW DEEP IS PRICE NOW?    As a share of the move. Zero at the end of it,
                            one back at its start.

  Past one, the number keeps counting. It says the move was undone and then
  some, which is true and worth knowing rather than clamping away.


THE FILES

  mod.rs            The front door.

  retracement.rs    FibRetracement. The two ends of the move, when each
                    happened, and when the pair could first have been drawn.

  tests.rs          Eight tests.

  README.txt        This file.


WHAT GETS REFUSED

  A move that went nowhere — nothing can be a share of nothing.

  A move that ends before it starts.

  And using one before both ends confirmed. Drawing levels off a move whose
  second swing had not confirmed is drawing them off a move that had not
  happened. Call is_known_at first.


WHICH RATIOS MATTER

  Not in here. They are in config/ta.toml, because which levels get used and
  what each is for is a trading decision — and a level with no job attached is
  a line the bot draws that nothing reads.
