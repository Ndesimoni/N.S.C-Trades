-- ── A reason belongs to a setup he TURNED DOWN ─────────────────────────────
--
-- His call, 3 September 2026: "we should only explain the why when we reject.
-- If we take a setup there should be no why. But if we reject it, then we tell
-- the why."
--
-- It is the right way round, and it makes the data sharper rather than
-- smaller. TAKING A SETUP MEANS THE RULES WERE RIGHT -- the sentence on the
-- card already says everything about why, and a note would only repeat it.
--
-- SKIPPING IS THE INFORMATION. It means the rules produced something he did
-- not want, and the reason is the one thing no measurement can supply. Those
-- are the "don't take this" examples the Phase 4 model needs, and they are
-- worthless without the why.
--
-- ENFORCED HERE AND NOT ONLY IN THE CODE, because it is a rule about what the
-- data MEANS. A note on a setup he took is not a mistake to be tidied up
-- later; it is a row nobody could interpret.
-- ───────────────────────────────────────────────────────────────────────────

-- Nothing has taken a note yet, but do it in the right order anyway: a
-- constraint that cannot be added because of existing rows is a migration that
-- fails on his machine and not on mine.
UPDATE signal_labels SET note = NULL WHERE verdict = 'took it';

ALTER TABLE signal_labels
    ADD CONSTRAINT why_is_for_the_ones_he_turned_down
    CHECK (note IS NULL OR verdict <> 'took it');
