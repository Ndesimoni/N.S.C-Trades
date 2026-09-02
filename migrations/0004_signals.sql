-- ── What the bot saw, and what it refused ──────────────────────────────────
--
-- Two tables and they are two halves of ONE dataset: what to take, and what
-- not to. A different shape on each side makes them unusable together, which
-- is the only use either has.
--
-- CLAUDE.md has asked for the second one since the beginning: "Rejected setups
-- get saved, not thrown away. Save which layer rejected them."
--
-- THE QUESTION THEY ANSWER: "why did nothing fire this week?" A quiet week and
-- a broken bot look identical today. "Nothing printed" and "forty shapes
-- printed and every one was thrown out at the place test" are completely
-- different problems, and right now they are the same silence.
--
-- Design: docs/worksheets/database.md, tables 4 and 7
-- ───────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS signals (
    id                BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    -- ── THREE MOMENTS, AND THEY ARE NOT THE SAME THING ──
    --
    -- On the four candlestick shapes he trades, `spans_from` and
    -- `candle_opened_at` are one and two candles apart -- three on a march.
    -- They are separate columns because a CHART pattern spans forty candles,
    -- and `at` is when the bot could first KNOW, which on those is days after
    -- the shape's last point printed.
    --
    -- Record the wrong one and every backtest enters early, at a price nobody
    -- could have traded. It will not error. It will look BETTER.
    at                TIMESTAMPTZ    NOT NULL,
    spans_from        TIMESTAMPTZ    NOT NULL,
    candle_opened_at  TIMESTAMPTZ    NOT NULL,

    symbol            TEXT           NOT NULL,
    interval          TEXT           NOT NULL,

    shape             TEXT           NOT NULL,
    shape_kind        TEXT           NOT NULL,
    direction         TEXT           NOT NULL,

    band_timeframe    TEXT           NOT NULL,
    band_price        NUMERIC(18, 8) NOT NULL,
    -- ── NOT `placing`, AND POSTGRES DECIDED THAT ──
    --
    -- PLACING is a fully reserved word -- it belongs to
    -- OVERLAY(... PLACING ... FROM ...) -- so as a bare column name it is a
    -- syntax error, and the parser blames the line AFTER it.
    --
    -- Quoting it everywhere would work and would be a trap: one query that
    -- forgets the quotes fails, and it would fail at runtime because these
    -- queries are checked at runtime by design.
    --
    -- `sits` is also what the code calls it -- nsc-strategy::place has
    -- `where_it_sits`. inside / just above / just below.
    sits              TEXT           NOT NULL,
    broke_out         BOOLEAN        NOT NULL,

    -- How big the shape was, in normal candles. On the card as how plainly
    -- the thing happened.
    reach             NUMERIC(18, 8) NOT NULL,

    -- ── STORED, NEVER WORKED OUT AGAIN ──
    --
    -- `sentence` is what he actually read on his phone. The wording will
    -- change, and a row that regenerated it would quietly rewrite history.
    --
    -- `features` is everything the bot saw at that moment. Recalculated later
    -- against updated chart-reading code, it trains a model on inputs the live
    -- bot never produced -- and NOTHING DETECTS THAT. Both sides keep working
    -- and only the scores are wrong.
    sentence          TEXT           NOT NULL,
    features          JSONB          NOT NULL,
    features_version  SMALLINT       NOT NULL,

    -- A hash of the config that produced it. Without this, "these came back at
    -- 38%" is unanswerable -- 38% under WHICH thresholds? The bot already
    -- refuses to reload settings while running so that this question has an
    -- answer; this is that promise written down.
    rules_version     TEXT           NOT NULL,

    -- Null means Telegram refused it. The bot still saw it.
    sent_at           TIMESTAMPTZ,

    -- ONE SHAPE, ONE CANDLE, ONE ZONE, ONE ROW. It also stops a restart
    -- writing the same signal twice -- the look runs again on every poll until
    -- the next candle closes.
    CONSTRAINT one_signal_per_candle_per_zone
        UNIQUE (symbol, interval, band_price, candle_opened_at)
);

ALTER TABLE signals
    ADD CONSTRAINT signal_interval_is_canonical
    CHECK (interval IN ('5m', '15m', '30m', '1h', '4h', '1d', '1w'));

-- Only candlesticks exist today. Chart patterns need trendlines, and nsc-ta
-- has none -- the value is here so the column never has to be added later.
ALTER TABLE signals
    ADD CONSTRAINT signal_kind_is_known
    CHECK (shape_kind IN ('candlestick', 'chart'));

ALTER TABLE signals
    ADD CONSTRAINT signal_direction_is_known
    CHECK (direction IN ('up', 'down'));

-- Read forward in time when a backtest measures outcomes.
CREATE INDEX IF NOT EXISTS signals_in_order ON signals (at);


-- ───────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS rejections (
    id                BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    at                TIMESTAMPTZ    NOT NULL,
    candle_opened_at  TIMESTAMPTZ    NOT NULL,
    symbol            TEXT           NOT NULL,
    interval          TEXT           NOT NULL,

    -- ── THE LAYER IS THE WHOLE POINT ──
    --
    -- shape   -- something printed and it is not one of his four
    -- place   -- one of his four printed, with no level under it
    -- measure -- too few candles to measure it
    --
    -- A CANDLE WITH NO SHAPE ON IT IS NOT WRITTEN DOWN. That is nearly every
    -- candle, it would make this table larger than `candles` while saying
    -- less, and it can be worked out from the candle any time. What cannot be
    -- worked out afterwards is the rest, because it depends on the settings
    -- that were live at that moment -- and those change.
    layer             TEXT           NOT NULL,
    why               TEXT           NOT NULL,

    -- The same shape as signals.features, exactly. They are the two halves of
    -- one dataset and a different shape on each side makes them unusable
    -- together.
    features          JSONB          NOT NULL,
    features_version  SMALLINT       NOT NULL,
    rules_version     TEXT           NOT NULL,

    -- The look runs again on every poll until the next candle closes, so the
    -- same refusal arrives many times. This is what makes writing it harmless.
    CONSTRAINT one_rejection_per_candle_per_layer
        UNIQUE (symbol, interval, candle_opened_at, layer)
);

ALTER TABLE rejections
    ADD CONSTRAINT rejection_interval_is_canonical
    CHECK (interval IN ('5m', '15m', '30m', '1h', '4h', '1d', '1w'));

ALTER TABLE rejections
    ADD CONSTRAINT rejection_layer_is_known
    CHECK (layer IN ('shape', 'place', 'measure', 'skip', 'direction', 'trigger'));

CREATE INDEX IF NOT EXISTS rejections_in_order ON rejections (at);

COMMENT ON TABLE rejections IS
    'It will grow far faster than signals. Partition by month, or prune rows '
    'older than a year once the Phase 4 training set is built.';
