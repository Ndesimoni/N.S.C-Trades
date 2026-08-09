-- What YOU thought of each signal.
--
-- Two sources fill this in:
--   1. the 👍/👎 buttons in Telegram, for live signals
--   2. the chart replay tool, for old setups (Phase 4)
--
-- Why keep this separate from the result? Because they answer different
-- questions. The result says whether the trade won. This says whether you
-- would have taken it.
--
-- A model trained only on results learns to chase winners, including ones you
-- would never have entered. Trained on both, it learns your judgement — which
-- is the actual goal.
CREATE TABLE signal_labels (
    id          BIGSERIAL PRIMARY KEY,
    signal_id   UUID        NOT NULL REFERENCES signals(id) ON DELETE CASCADE,

    -- 'would_take' | 'would_skip'
    verdict     TEXT        NOT NULL,
    -- Optional: WHY you skipped it. These notes are how you find rules missing
    -- from config/strategy.toml — every skip you cannot explain with an
    -- existing rule is a rule you have not written down yet.
    note        TEXT,

    -- 'telegram_button' | 'replay_tool' | 'import'
    source      TEXT        NOT NULL,
    labelled_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- One verdict per signal per source. Pressing the button again replaces
    -- the old answer rather than adding a second one.
    UNIQUE (signal_id, source)
);

CREATE INDEX labels_verdict_idx ON signal_labels (verdict);
