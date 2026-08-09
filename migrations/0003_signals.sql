-- Every setup the bot found — including the ones it never sent.
--
-- The blocked ones matter as much as the sent ones. They are the "don't take
-- this" examples the Phase 4 model needs. A dataset of only sent signals
-- teaches a model nothing about what to refuse.
CREATE TABLE signals (
    id              UUID PRIMARY KEY,
    symbol_id       SMALLINT    NOT NULL REFERENCES symbols(id),
    timeframe       TEXT        NOT NULL,
    direction       TEXT        NOT NULL,      -- 'long' | 'short'

    -- Which candle close produced this. Together with the pair and timeframe
    -- this makes the setup reproducible: replay to this candle and the bot
    -- must produce exactly the same thing.
    bar_time        TIMESTAMPTZ NOT NULL,

    entry           NUMERIC(18,8) NOT NULL,
    stop_loss       NUMERIC(18,8) NOT NULL,
    take_profit     NUMERIC(18,8) NOT NULL,
    risk_reward     NUMERIC(8,3)  NOT NULL,
    suggested_lots  NUMERIC(10,4),

    confluence_score SMALLINT    NOT NULL,
    -- The plain-English "why", shown in your Telegram message.
    reasons         JSONB       NOT NULL,
    -- Everything the bot saw at that moment. This is what the Phase 4 model
    -- trains on, so it is saved exactly as it was rather than worked out
    -- again later — recalculating against updated code would train the model
    -- on inputs the live bot never actually produced.
    features        JSONB       NOT NULL,

    -- 'sent' | 'suppressed_score' | 'suppressed_news' | 'suppressed_risk'
    --        | 'suppressed_cooldown' | 'suppressed_ai'
    status          TEXT        NOT NULL,
    suppressed_by   TEXT,

    strategy_name    TEXT       NOT NULL,
    strategy_version INT        NOT NULL,
    -- Empty for live signals. Filled in when it came from a backtest.
    backtest_run_id  UUID,

    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX signals_symbol_time_idx ON signals (symbol_id, bar_time DESC);
CREATE INDEX signals_status_idx      ON signals (status, created_at DESC);
CREATE INDEX signals_backtest_idx    ON signals (backtest_run_id) WHERE backtest_run_id IS NOT NULL;

-- Stops the same idea being sent over and over on consecutive candles. Redis
-- handles the live cooldown; this is the permanent backstop.
CREATE UNIQUE INDEX signals_dedup_idx
    ON signals (symbol_id, timeframe, direction, bar_time)
    WHERE backtest_run_id IS NULL;
