# fixtures/

Saved candle files for tests. Committed to git on purpose.

The clean crates make a strong promise: the same candles always produce the
same swings, the same levels and the same signals, forever.

That promise is only worth something if you test it against input that cannot
change — which a live broker connection is not.

## The format

A CSV with a header row. Column names are matched case-insensitively and the
order does not matter — the header says which column is which.

```
time,open,high,low,close
2025-01-06,1.20000,1.20740,1.19820,1.20560
```

Accepted names for the first column: `time`, `timestamp`, `date`, `datetime`,
`open_time`. A `volume` column is ignored — cash forex and CFDs have no traded
volume, so nothing in this project may depend on it.

Timestamps mark when the candle **started**, and are read as **UTC** unless
they carry an offset of their own. If your export is in broker time, convert it
before it gets here: guessing a timezone would shift every candle, and with it
every level the bot draws.

**Watch the last row.** A file exported mid-session ends with a candle that is
still forming — its high and low have not finished happening. Nothing in the
file says so, and the reader will not pretend to know.

## What is here

```
fixtures/candles/EURUSD_M15_2024Q1.csv     a clean trending quarter
fixtures/candles/EURUSD_M15_range.csv      chop — where simple levels fall apart
fixtures/candles/GBPJPY_M15_volatile.csv   proves settings really are in ATR,
                                           not pips in disguise
fixtures/candles/USDJPY_M15_gapped.csv     deliberate holes, for the gap checker
```

## Golden files

The expected answer is saved next to each input. When a golden file changes,
the difference tells you exactly which levels moved and why.

That is the only practical way to review a change to swing detection, because
every level, trendline, Fibonacci anchor and trend reading shifts at once.

**Regenerate these deliberately, never out of habit.** A regenerated file
nobody read is a silently accepted change to the part of the system everything
depends on.
