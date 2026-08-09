-- The pairs you trade.
-- Mirrors config/symbols.toml, but stored in the database so old rows keep
-- the pip size they were created with even if you edit the settings later.
CREATE TABLE symbols (
    id              SMALLSERIAL PRIMARY KEY,
    name            TEXT        NOT NULL UNIQUE,   -- 'EURUSD'
    base_currency   CHAR(3)     NOT NULL,
    quote_currency  CHAR(3)     NOT NULL,
    pip_size        NUMERIC(12,8) NOT NULL,
    digits          SMALLINT    NOT NULL,
    max_spread_pips NUMERIC(6,2),
    active          BOOLEAN     NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
