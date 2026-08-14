
The market is closed at the weekend. These candles are built from indicative
quotes and their range is about **one hundredth** of a real day.

Two things break if they are kept:

1. **Your chart has 5 daily candles a week. This feed has 7.** The bot would
   not be looking at your chart.
2. They sit between Friday's close and Monday's open, so a swing finder reads
   them as price pausing and consolidating — and invents a swing that nobody
   traded.

**Rule: a daily candle stamped Saturday or Sunday is not a trading day. Drop
it before anything reads it.**

**Not yet checked:** whether hourly candles are produced over the weekend too,
and whether they are equally flat.

---

## The newest candle is always still forming

Asked at 15:50 UTC, the newest hourly candle was stamped 15:00 — an hour that
had ten minutes left to run. Its high was not its high and its close was not
its close.

**It must never reach the analysis.**

And the fix is not "skip the first one". Ask again at 16:00:02 and you get
either:

- the 16:00 candle already open, if a price has arrived — the finished one is
  second in the list
- the 15:00 candle, now finished, if no price has arrived yet — the finished
  one is first

Position is not reliable. **Time is.** Take the newest candle whose open time
plus one interval is not in the future.

---

## Smaller things worth knowing

**No volume.** Spot forex and gold have no central exchange to count it. No
rule in this project may ever use volume.

**Prices arrive as text** — `"4394.68931"` — which is helpful. They go straight
into `Decimal` and never touch a float.

**Five decimal places.** Your chart shows gold to two. Five means this is
several sources blended, not one broker's book, so prices will be *close to*
your chart rather than identical. Fine for levels, because a level is a band.
Worth remembering when a number looks slightly off.

**Errors come back as normal JSON, not an HTTP failure.** A bad key gives
`{"code": 401, "status": "error", "message": "..."}` with a perfectly ordinary
response. So a reply that parses is not the same as a reply that worked.

---

## Still unknown

- Does the day boundary follow New York through the clock change? Re-check in
  November.
- Are there weekend hourly candles, and are they flat too?
- What "8 trial WS" actually allows — how many symbols, and for how long.
- Whether gold's nightly hour off (17:00 to 18:00 New York) shows up here as a
  gap, as it does in the broker exports.
