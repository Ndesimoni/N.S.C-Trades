-- ── What HE thought of a signal ────────────────────────────────────────────
--
-- Two buttons under every setup: took it, skipped it. His yes, 30 August 2026.
--
-- THIS IS THE TABLE THAT CANNOT BE RECREATED. Candles can be downloaded again
-- and outcomes recomputed from them. What he thought of a setup on the
-- afternoon it printed exists nowhere else the moment he forgets it.
--
-- It also has to fill itself. A table that needs a sitting nobody schedules is
-- a table that stays empty, which is why it is buttons and not a form.
--
-- Design: docs/worksheets/database.md, table 6
-- ───────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS signal_labels (
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    signal_id   BIGINT      NOT NULL REFERENCES signals (id) ON DELETE CASCADE,
    at          TIMESTAMPTZ NOT NULL,

    -- took it        / skipped it   -- a button, at the time
    -- would have     -- said later, in words, once the outcome came in
    verdict     TEXT        NOT NULL,

    -- In his own words. Null when it came from a button and he has not said
    -- anything about it yet.
    note        TEXT,

    -- ── ONE VERDICT PER SIGNAL, AND IT CAN BE CHANGED ──
    --
    -- Telegram RESENDS a callback when it does not hear back, so a single tap
    -- arrives more than once. Without this, one tap becomes three rows saying
    -- the same thing.
    --
    -- Tapping the other button REPLACES the verdict rather than adding a row.
    -- He is allowed to change his mind, and `at` moves with it so the record
    -- says when he settled rather than when he first wavered.
    CONSTRAINT one_verdict_per_signal UNIQUE (signal_id)
);

ALTER TABLE signal_labels
    ADD CONSTRAINT verdict_is_known
    CHECK (verdict IN ('took it', 'skipped it', 'would have skipped'));

COMMENT ON COLUMN signal_labels.verdict IS
    'WOULD HAVE SKIPPED HAS NO BUTTON, and that is deliberate. It is what he '
    'says later, in words, when the outcome came in -- a third button would '
    'invite him to answer before the market had.';
