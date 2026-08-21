candles/ — where IBKR starts its day
====================================


WHAT THIS IS FOR

      cargo run -p nsc-work-man --bin candles              XAU/USD
      cargo run -p nsc-work-man --bin candles -- EUR/USD

  TWO QUESTIONS. The second one is the dangerous one.

  ARE THE NUMBERS RIGHT? It prints the last daily and weekly candles. Put the
  highs and lows next to his chart and look.

  WHERE DOES IBKR START ITS DAY? config/when.toml says the day ends 17:00 New
  York. THAT WAS MEASURED ON TWELVE DATA, and the feed changed on 20 August
  2026. Nobody has checked IBKR.


WHY IT MATTERS MORE THAN IT SOUNDS

  If IBKR ends its day somewhere else, every daily candle is a DIFFERENT
  CANDLE -- different open, different high, different range.

  Band thickness is 0.46 of a normal daily candle. So every daily band changes
  size, every daily level moves, and every alert fires in a slightly wrong
  place.

  NOTHING ERRORS. The candles come back perfectly well. They are just not the
  candles on his chart, and the first thing he would notice is a level in the
  wrong place weeks later.


HOW IT MEASURES, WITHOUT TRUSTING ANYBODY'S DOCUMENTATION

  A daily candle's open IS an hourly candle's open -- the same tick, written
  down twice. So the hour that shares the number is the hour the day began.

  Same again one level up: the day that shares the weekly candle's open is the
  day the week began.

  NOTHING HERE WORKS A BOUNDARY OUT BY ARITHMETIC. That is the mistake this
  whole check exists to catch. Guessing wrong reads a candle before the market
  printed it, and that does not error -- it makes results look better.


EACH BOUNDARY IS TESTED AGAINST THE STEP BELOW IT

  The day against hours. The week against days.

  AND THERE MUST BE ENOUGH OF THE SMALLER ONES to cover every big candle being
  tested. That was wrong in the first version: it fetched six daily candles
  and then lined five WEEKLY candles up against them. Six days is barely one
  week, so four of the five could never match -- and it would have said NOT
  SETTLED, which reads as a finding about IBKR rather than a fault in the
  sample.

  So: 200 hourly candles for six days, and 40 daily candles for six weeks.


IT REFUSES TO ANSWER ON THIN EVIDENCE

  ONE CANDLE MATCHING IS A COINCIDENCE. A quiet market opens two hours on the
  same number often enough. So it lines up six days and only answers if every
  one of them agrees on the same hour.

  If they disagree, or if two hours share an open, it says NOT SETTLED and
  tells you to run it again while the market is open. A measurement taken on a
  shut market has already cost this project one wrong answer -- gold measured
  on a Saturday said it moves 0.73 an hour instead of 13.33.


THE FILES

  main.rs      Which pair, fetching, and printing what it found.

  boundary.rs  Lining the big candles up against the small ones, and deciding
               whether they agree. Four tests.

  README.txt   This file.
