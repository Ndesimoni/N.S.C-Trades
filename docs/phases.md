# What gets built, and in what order

Each phase has a finish line. Move on before you reach it and you end up
debugging two things at once, which costs more time than waiting would have.

## Phase 0 — the boring foundations

Download price history. Store it. Build the backtester. Turn on the
lookahead checks.

**Finished when:** you can replay a year of EURUSD 15-minute candles and the
lookahead checks stay quiet.

## Phase 1 — reading the chart

Swing highs and lows first. Then levels, trend direction, Fibonacci,
trendlines, candlestick patterns, ATR.

**Finished when:** the levels it draws match the ones you would draw yourself,
on charts it has never seen.

Chart patterns — head and shoulders, triangles, flags — are deliberately
**not** in this phase. Trend direction, levels, Fibonacci and candlesticks
give you most of the edge for a fraction of the work.

## Phase 2 — your rules

Write your strategy into `config/strategy.toml`. Send signals to Telegram.
No trading.

**Finished when:** you understand every signal it produces. Not when the
results are good — when you can explain them.

## Phase 3 — collecting the data

Run it live. Press 👍 or 👎 on every signal. Let the tracker record how each
one turned out.

**Finished when:** you have 200–300 signals you've judged.

This is the phase people skip, and skipping it makes Phase 4 impossible.

It is also where you find out which rules you follow without realising. That
is worth more than the model you build afterwards.

## Phase 4 — teaching it your judgement

Export your labels. Train a model in `research/`. Find out which of your
confluences actually matter. Load the model back into the bot.

**Finished when:** the model beats simply taking every signal above your
quality threshold, tested on data it has not seen.

If it does not beat that, you need more labels, not a bigger model.

Expect to find that two of your confluences do nearly all the work and the
rest are decoration. That finding is the real prize here.

## Phase 5 — news and AI checks

Block trades around big news. Read headlines. Ask an AI for a second opinion.

**Finished when:** the news filter measurably improves your results in
backtest.

If it does not, leave it switched off. A filter that does not help is not
free — it costs you trades.

## Phase 6 — actually trading (version 2)

Crypto first. Crypto exchanges have clean, simple connections and run 24/7.
Forex needs a bridge through MetaTrader and probably a Windows server.

Forex trading comes after — and only once the signals have a track record you
would have been happy to trade by hand.
