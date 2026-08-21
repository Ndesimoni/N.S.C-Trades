indicators/ — numbers off the price series
==========================================


WHAT THIS FOLDER IS FOR

  Things worked out from a run of prices rather than from the shape of one
  candle.


THE FILES

  mod.rs      The front door.
  fibonacci/  Retracements over a move. Its own README.
  README.txt  This file.


EVERYTHING IS A SHARE OR IN ATR, NEVER IN POINTS

  A three-point pullback is nothing on gold and a week on the euro. A points
  threshold works on the pair it was set on and quietly stops working on every
  other one.


WHAT IS NOT HERE

  No RSI, no MACD, no moving averages, no Bollinger bands. None of them are
  written and none are planned yet.

  Worth saying why: an indicator reading floating in mid-chart predicts about
  as much as a candlestick pattern floating in mid-chart does, and that was
  MEASURED on 20 August 2026 -- 96 of 136 pattern results sat inside their own
  noise band. See bin/after/.

  What makes a reading worth anything in this project is the LEVEL it happens
  at. Adding indicators before that is wired up buys nothing.
