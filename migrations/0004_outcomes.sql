-- What the market actually did after each signal.
--
-- Filled in by the tracker job, which follows price forward until it hits the
-- stop or the target. This is the OBJECTIVE half. What you thought of it is
-- in the next table.
CREATE TABLE signal_outcomes (
    signal_id       UUID PRIMARY KEY REFERENCES signals(id) ON DELETE CASCADE,

    -- 'open' | 'target_hit' | 'stop_hit' | 'expired' | 'ambiguous'
    --
    -- 'ambiguous' means price hit both the stop and the target inside one
    -- candle, so the data genuinely cannot say which came first. These get
    -- left out of the numbers rather than guessed at.
    result          TEXT        NOT NULL,
    resolved_at     TIMESTAMPTZ,

    -- The result as a multiple of what you risked. The only unit worth
    -- comparing across pairs, because it accounts for stop distance and pip
    -- value.
    r_multiple      NUMERIC(8,3),

    -- How far price ran your way, and how far it ran against you, before
    -- resolving. These answer: are my stops too tight, are my targets too
    -- greedy?
    max_favourable_r NUMERIC(8,3),
    max_adverse_r    NUMERIC(8,3),
    bars_to_resolve  INT,

    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX outcomes_result_idx ON signal_outcomes (result);
