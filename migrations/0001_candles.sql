-- ── The candles ────────────────────────────────────────────────────────────
--
-- The history everything else is measured against. Years of it, read forward,
-- streamed.
--
-- Design: docs/worksheets/database.md
-- ───────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS candles (
    symbol      TEXT           NOT NULL,
    interval    TEXT           NOT NULL,
    opened_at   TIMESTAMPTZ    NOT NULL,

    open        NUMERIC(18, 8) NOT NULL,
    high        NUMERIC(18, 8) NOT NULL,
    low         NUMERIC(18, 8) NOT NULL,
    close       NUMERIC(18, 8) NOT NULL,

    tick_volume BIGINT,
    source      TEXT           NOT NULL,

    -- ── THE PRIMARY KEY IS THE INDEX THE BACKTEST READS ON ──
    --
    -- Forward in time, one pair, one timeframe -- exactly the order this key
    -- stores. A separate index would be a second copy of the same thing.
    PRIMARY KEY (symbol, interval, opened_at),

    -- ── NEVER A FLOAT, AND THESE CHECKS SAY WHY IT MATTERS ──
    --
    -- 0.1 + 0.2 is not 0.3 in floating point, and a level at 4520.00 stored
    -- as 4519.9999998 answers "did price touch it" with NO while his eye says
    -- yes. NUMERIC everywhere a price appears.
    --
    -- A candle whose high is under its low is not a candle. It has never been
    -- seen from IBKR; it costs nothing to make impossible.
    CONSTRAINT candle_is_a_candle CHECK (high >= low),
    CONSTRAINT high_holds_the_body CHECK (high >= open AND high >= close),
    CONSTRAINT low_holds_the_body  CHECK (low  <= open AND low  <= close)
);

-- ── Enums are TEXT with a CHECK, not Postgres enums ────────────────────────
--
-- A Postgres enum needs a migration and an exclusive lock to add a value. A
-- CHECK is a one-line ALTER. Adding a timeframe should not be a schema event.
-- ───────────────────────────────────────────────────────────────────────────

ALTER TABLE candles
    ADD CONSTRAINT interval_is_canonical
    CHECK (interval IN ('5m', '15m', '30m', '1h', '4h', '1d', '1w'));

-- **The feed's own spelling never gets in here.** This project already paid
-- for that once: the timeframe travelled as a &'static str, two spellings of
-- the same thing became two keys, and the same candle was reported twice.

ALTER TABLE candles
    ADD CONSTRAINT source_is_named
    CHECK (source IN ('ibkr'));

COMMENT ON COLUMN candles.opened_at IS
    'WHEN THE CANDLE OPENED, never when it ended. On the old feed an hourly '
    'stamp was the open and a daily stamp was the date it ended on. Storing '
    'the open means that conversion happens once, on the way in, where it can '
    'be tested.';
