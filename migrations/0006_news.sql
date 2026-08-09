-- Economic calendar and headlines, used for news blackouts and as background
-- for the AI check.
CREATE TABLE news_events (
    id            BIGSERIAL PRIMARY KEY,
    external_id   TEXT UNIQUE,               -- the provider's id, so syncing twice is safe
    event_time    TIMESTAMPTZ NOT NULL,
    currency      CHAR(3)     NOT NULL,      -- which currency it affects
    title         TEXT        NOT NULL,      -- 'Non-Farm Payrolls'
    impact        TEXT        NOT NULL,      -- 'high' | 'medium' | 'low'
    actual        TEXT,
    forecast      TEXT,
    previous      TEXT,
    source        TEXT        NOT NULL,
    fetched_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- The blackout check runs on every candle close, so this index earns its keep.
CREATE INDEX news_time_currency_idx ON news_events (event_time, currency, impact);

-- Headlines, turned into structured information by an AI.
-- Background only: headlines never create a signal, they only add context or
-- support a skip.
CREATE TABLE news_headlines (
    id             BIGSERIAL PRIMARY KEY,
    external_id    TEXT UNIQUE,
    published_at   TIMESTAMPTZ NOT NULL,
    headline       TEXT        NOT NULL,
    url            TEXT,
    -- What the AI made of it: which currencies, which direction, how serious.
    classification JSONB,
    classified_at  TIMESTAMPTZ
);

CREATE INDEX headlines_published_idx ON news_headlines (published_at DESC);
