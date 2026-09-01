-- ── The economic calendar ──────────────────────────────────────────────────
--
-- What ForexFactory said was coming, as at the last time we asked.
--
-- His call, 1 September 2026: download the week every six hours and keep it
-- HERE, rather than in a Vec that dies with the process.
--
-- Three things that buys, and the first is the one that lasts:
--
--   * A RECORD. Nothing remembered what the calendar said, so no backtest
--     could ever ask "was there news within ten minutes of this setup?" --
--     which is a question worth asking before trusting any result.
--   * A restart works instantly instead of waiting on a download.
--   * The feed being unreachable at startup stops meaning no news warnings.
--
-- Design: docs/worksheets/database.md
-- ───────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS news_events (
    -- ── THE KEY IS THE SAME ONE THE BOT USES IN MEMORY ──
    --
    -- Event::key() is time, currency and title, and that is what decides
    -- whether he has already been told about a release. Two different answers
    -- to "is this the same event" is how one release becomes two cards.
    at          TIMESTAMPTZ NOT NULL,
    currency    TEXT        NOT NULL,
    title       TEXT        NOT NULL,

    impact      TEXT        NOT NULL,

    -- Kept as the feed's own text, empty when there is none. A speech has
    -- neither. Parsing them into numbers here would turn "<0.1%" into a
    -- guess, and the card only ever prints them.
    forecast    TEXT        NOT NULL DEFAULT '',
    previous    TEXT        NOT NULL DEFAULT '',

    -- ── WHEN WE FIRST SAW IT, AND WHEN WE LAST DID ──
    --
    -- The feed revises the week while it is running: forecasts land, tentative
    -- times firm up, events are added. last_seen is how a row that has fallen
    -- out of the file is told from one that is still listed -- see the delete
    -- in store/news.rs. first_seen is kept because "this was added on
    -- Wednesday" is worth knowing later.
    first_seen  TIMESTAMPTZ NOT NULL,
    last_seen   TIMESTAMPTZ NOT NULL,

    PRIMARY KEY (at, currency, title)
);

-- ── Enums are TEXT with a CHECK, not Postgres enums ────────────────────────
--
-- Same reason as candles: a Postgres enum needs a migration and an exclusive
-- lock to add a value, a CHECK is a one-line ALTER.
--
-- These are Impact's own spellings, NOT the feed's. The feed's raw text never
-- gets in here -- it is matched without case on the way in, so a change of
-- case at their end cannot quietly empty the filter.
-- ───────────────────────────────────────────────────────────────────────────

ALTER TABLE news_events
    ADD CONSTRAINT impact_is_canonical
    CHECK (impact IN ('High', 'Medium', 'Low', 'Holiday', 'Unknown'));

-- Holiday and Unknown are in that list on purpose. THE WHOLE FILE IS STORED,
-- not only the ratings he wants a card for -- which of them earn a message is
-- config/news.toml's business and it can change without a migration. A
-- holiday is also worth having: a thin session explains a day that behaved
-- oddly, and it is not a release so nothing prints.

-- The bot reads a window: everything between now and a few hours out. That is
-- a range scan on `at`, and `at` leads the primary key, so it is already the
-- index for it. Nothing extra to add.

COMMENT ON COLUMN news_events.at IS
    'WHEN IT PRINTS, IN UTC. The feed stamps these with a New York offset and '
    'it is converted once, on the way in -- same rule as candles. A release '
    'that MOVES becomes a new row, and the old one stops being listed and is '
    'deleted on the next download.';
