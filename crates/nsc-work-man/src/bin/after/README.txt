after/ — what price actually did next
=====================================


WHAT THIS IS FOR

      cargo run -p nsc-work-man --bin after -- XAU/USD 4h 1200
      cargo run -p nsc-work-man --bin after -- EUR/USD 1h 4400

  Finds every pattern in the history, then looks at what price did 1, 3, 5 and
  10 candles later.


THE FILES

  main.rs     Fetching, walking the history, and printing.
  outcome.rs  What "it went its way" means, and by how much.
  README.txt  This file.


THE BASE RATE IS THE WHOLE POINT

  A pattern that is right 55% of the time is worth NOTHING if price rose 55%
  of the time anyway.

  Every candlestick page on the internet quotes the first number and not the
  second. So this prints the DIFFERENCE, and never the raw rate.

  TWO CONTROLS, NOT ONE. Gold drifted up this year, so an ordinary candle
  "claiming up" was right 51% of the time and one claiming down only 49%.
  Judged against the up rate, every bearish pattern starts two points behind
  before it has done anything. Each pattern is set against the control facing
  its own way.


AND THE NOISE COLUMN IS THE SECOND POINT

  On 200 tries a fair coin lands 3.5 points either side of 50% as a matter of
  course. A pattern "beating the market by 3" on 200 samples is a pattern
  doing nothing.

  So every row carries how far a coin would stray on that many flips -- one
  standard error. A NUMBER SMALLER THAN ITS OWN NOISE COLUMN SAYS NOTHING.

  There are forty-eight numbers in the table. About a third will clear one
  standard error by luck alone. Do not go looking for the big ones.


CLOSES, NOT HIGHS AND LOWS

  A high that was touched and given straight back is not a move he could have
  taken. Counting it is the surest way to build a pattern that works
  beautifully and cannot be traded.


WHAT THIS IS NOT

  IT IS NOT A BACKTEST. No spread, no slippage, no stop, no target, no
  position size. It answers one question -- did price tend to go the way the
  pattern claimed -- and nothing else.

  IT BELONGS IN nsc-backtest when that crate exists.

  AND IT IGNORES THE LEVEL, which is the thing this whole project says makes a
  pattern mean anything. It measures patterns everywhere, which is exactly the
  reading the design already calls worthless. Read the result as a check on
  that claim, not as a verdict on the bot.
