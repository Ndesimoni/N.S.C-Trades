-- One row per backtest run, so results stay comparable as the rules and the
-- chart-reading code change.
--
-- The git_sha column is not paperwork. A backtest number means nothing without
-- knowing which version of the chart-reading code produced it. Comparing two
-- runs across a change to swing detection is comparing two different systems.
CREATE TABLE backtest_runs (
    id            UUID PRIMARY KEY,
    label         TEXT,
    started_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at   TIMESTAMPTZ,

    period_start  TIMESTAMPTZ NOT NULL,
    period_end    TIMESTAMPTZ NOT NULL,
    symbols       TEXT[]      NOT NULL,

    -- A full copy of the settings used: strategy.toml, ta.toml, risk.toml.
    -- Without this, a promising run can never be repeated.
    config        JSONB       NOT NULL,
    git_sha       TEXT,

    -- The results: trade count, win rate, average result, profit factor,
    -- worst drawdown, average time in a trade.
    metrics       JSONB,
    notes         TEXT
);

CREATE INDEX backtest_runs_started_idx ON backtest_runs (started_at DESC);
