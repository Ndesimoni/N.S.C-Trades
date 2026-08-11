.github/workflows/
==================

What GitHub runs for you automatically.

  ci.yml    Every push to main and every pull request. Two jobs:
            "Project rules" runs ci/rules.sh, and "Format, clippy, tests"
            runs the three cargo commands.

There is no deploy workflow, and that is on purpose. Nothing is deployed
yet — Phase 0 is unfinished, so there is no bot to ship and no server to
ship it to. A deploy pipeline written now would be built against a guess
about a machine that does not exist, and it would be wrong by the time it
was needed.

Every check here can also be run on your own machine. See ci/README.txt.
That file is the one to read: it explains what each check is for and what
breaks without it. This file only says when they run.
