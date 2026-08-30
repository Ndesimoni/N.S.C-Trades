-- ── UTC, and it is not decoration ──────────────────────────────────────────
--
-- TIMESTAMPTZ stores an absolute instant, so the DATA is right whatever the
-- server's clock is set to. What is wrong on a server set to local time is
-- everything a PERSON reads.
--
-- Found on 30 August 2026, looking at the first candle in the record on
-- Postgres.app. It showed:
--
--     2010-02-12 08:00:00+04
--
-- The candle opened at 04:00 UTC and the CSV said so. His Mac is on
-- Asia/Dubai, Postgres.app inherited it, and the screen said 08:00. Nothing
-- had shifted -- but the first person to read that screen would have believed
-- a 4-hour candle opened four hours late, and gone looking for a bug in the
-- feed.
--
-- The container in docker-compose.yml sets TZ and PGTZ. Postgres.app has no
-- such setting, so it goes on the database itself, where it holds whatever
-- machine or client connects.
-- ───────────────────────────────────────────────────────────────────────────

DO $$
BEGIN
    EXECUTE format('ALTER DATABASE %I SET timezone = %L', current_database(), 'UTC');
EXCEPTION
    -- **Not fatal.** A role that does not own the database cannot set this,
    -- and the bot sets it per connection anyway -- see store/pool.rs. Failing
    -- the migration here would stop a working bot over a display setting.
    WHEN insufficient_privilege THEN
        RAISE NOTICE 'could not set the database timezone; connections still force UTC';
END
$$;
