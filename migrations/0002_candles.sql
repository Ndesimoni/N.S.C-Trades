-- Price history.
--
-- Only the small candles come from the broker. The bigger ones are built in
-- our own code so that when a day ends stays under our control. They still
-- get saved here, so backtests do not rebuild them on every run.
--
-- open_time is when the candle STARTED, always UTC. Storing the close time
-- instead causes off-by-one-candle bugs that are very hard to spot.
CREATE TABLE candles (
    symbol_id   SMALLINT    NOT NULL REFERENCES symbols(id),
    timeframe   TEXT        NOT NULL,          -- 'M15' | 'H1' | 'H4' | 'D1'
    open_time   TIMESTAMPTZ NOT NULL,
    open        NUMERIC(18,8) NOT NULL,
    high        NUMERIC(18,8) NOT NULL,
    low         NUMERIC(18,8) NOT NULL,
    close       NUMERIC(18,8) NOT NULL,
    volume      NUMERIC(18,4),                 -- tick count in forex
    -- FALSE while the candle is still forming. Analysis MUST ignore these.
    -- Acting on a half-formed candle is a quiet way of using data you do not
    -- have yet, because it is not the candle that ends up in the history.
    complete    BOOLEAN     NOT NULL DEFAULT TRUE,
    PRIMARY KEY (symbol_id, timeframe, open_time)
);

-- The backtester reads forward through time, one pair at a time. That is the
-- pattern this index serves.
CREATE INDEX candles_scan_idx ON candles (symbol_id, timeframe, open_time DESC);

-- Where the feed had holes. Weekends are expected. Anything else is a data
-- problem that would quietly corrupt your analysis.
CREATE TABLE candle_gaps (
    id          BIGSERIAL PRIMARY KEY,
    symbol_id   SMALLINT    NOT NULL REFERENCES symbols(id),
    timeframe   TEXT        NOT NULL,
    gap_start   TIMESTAMPTZ NOT NULL,
    gap_end     TIMESTAMPTZ NOT NULL,
    expected_bars INT       NOT NULL,
    reason      TEXT,                          -- 'weekend' | 'holiday' | 'unknown'
    detected_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
