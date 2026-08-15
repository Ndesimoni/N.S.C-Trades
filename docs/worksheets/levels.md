# Levels

What a level is, how thick it is, and where the numbers came from.

---

## A level is a band, not a line

Price does not stop at a number. It turns somewhere near one.

So every level has a thickness, and "price is at the level" means price is
inside that band.

## The thicknesses

| Timeframe | Colour | Thickness | Measured? |
|---|---|---|---|
| Weekly | black | **0.35** of a weekly candle | ✅ twice |
| Daily | blue | **0.46** of a daily candle | ✅ once |
| 4-hour | yellow | 0.55 of a 4-hour candle | ❌ a guess |

**The same on every pair.** Whatever he sends, those are the thicknesses.

They live in `config/levels.toml`.

## Why a share of a candle and not a price

Write it as *78 points* and it is right on gold and absurd everywhere else —
78 points is a normal week on gold and about a year on EURUSD.

A share of a normal candle travels. 0.35 gives 78 on gold and about 0.004 on
EURUSD, and **both look the same on a chart** — which is what he is actually
doing when he draws one.

## Where the numbers came from

Measured off his own weekly gold chart on 15 August 2026. TradingView shows
the top and bottom of a drawn band on the price axis, so these are exact
rather than read off pixels.

```
weekly   4132.020 - 4055.913  =  76.11   0.35 of a weekly candle
weekly   3383.480 - 3303.553  =  79.93   0.36 of a weekly candle
daily    3000.463 - 2968.181  =  32.28   0.46 of a daily candle
```

Gold's candles at the time: weekly 220.42, daily 70.36, both over 14 periods.

**The weekly is the trusted one.** Two bands drawn months apart, at 4,094 and
at 3,343, landing on 0.35 and 0.36. That is a habit, not a coincidence.

**The daily replaced an older note.** That note said 0.60, from a different set
of levels on USDCAD before the project was cleared. His own hand says 0.46.

## One thing that is odd, and worth remembering

His two weekly bands are **76.1** and **79.9** points — nearly the same, drawn
when gold traded at 4,094 and at 3,343.

If he sized them against the candles in front of him, the lower one should be
noticeably thinner, because gold's candles were smaller back then. It is not.

**So he sizes by how it looks on screen, not by the candles.** On an ordinary
price axis, the same look means the same number of points.

The share-of-a-candle rule is how we *reproduce* what his eye does, and it
lands within 3%. But it is a model of his hand, not his hand — and as gold's
volatility changes, one day a band will look wrong to him. When that happens,
this is the paragraph to come back to.

## Still to settle

- **The 4-hour thickness has never been measured.** Replace the guess the
  first time a 4-hour level arrives with its edges visible.
- **Whether a band drifts.** These are worked out from the candle size *now*.
  A level drawn a year ago would get a different thickness today. Nobody has
  decided whether that is right.
