#!/usr/bin/env bash
# The project-specific checks. See ci/README.txt for what each one is for and
# what breaks without it. Run it yourself before pushing: ./ci/rules.sh
#
# No cargo, no network, no database. Reads files and greps. Under a second.
set -uo pipefail
cd "$(dirname "$0")/.."

FAILED=0

# Print a failure and remember that one happened. Everything keeps running,
# so one push shows you every problem instead of one at a time.
fail() {
    printf '  FAIL  %s\n' "$1"
    FAILED=1
}
heading() { printf '\n%s\n' "$1"; }

# The two crates that must stay pure: no outside world, ever.
CLEAN_CRATES="crates/nsc-ta crates/nsc-strategy"

# Everything except the two binaries. Binaries may panic; libraries may not.
LIB_CRATES="crates/nsc-core crates/nsc-ta crates/nsc-strategy crates/nsc-data
            crates/nsc-risk crates/nsc-news crates/nsc-ai crates/nsc-chart
            crates/nsc-telegram crates/nsc-backtest"

# Source files that are not tests. Test files are allowed to panic, and are
# allowed to write literal numbers, because that is what a fixture is.
non_test_sources() {
    find $1 -name '*.rs' -not -name 'tests.rs' -not -path '*/tests/*' 2>/dev/null
}

# A Cargo.toml with its comments stripped. nsc-core's own comments mention
# tokio and sqlx by name, so grepping the raw file finds the warning about
# the problem rather than the problem.
manifest_body() { sed 's/#.*//' "$1"; }

# ── 1. clean crates have no dirty dependencies ─────────────────────────────
heading "1. nsc-ta and nsc-strategy stay off the internet"
BANNED='tokio|sqlx|reqwest|redis|axum|tower-http|tokio-tungstenite|teloxide|futures|async-trait|plotters'
for crate in $CLEAN_CRATES; do
    hits=$(manifest_body "$crate/Cargo.toml" | grep -n -E "^[[:space:]]*($BANNED)[[:space:]]*=")
    [ -n "$hits" ] && fail "$crate/Cargo.toml depends on: $(echo "$hits" | tr '\n' ' ')"
done

# ── 2. no clock, no global state, no async ─────────────────────────────────
heading "2. nsc-ta and nsc-strategy read no clock and hold no global state"
IMPURE='Utc::now|Local::now|SystemTime::now|Instant::now|async fn|static mut|lazy_static|OnceLock|OnceCell|std::env'
for f in $(non_test_sources "$CLEAN_CRATES"); do
    hits=$(grep -n -E "$IMPURE" "$f")
    [ -n "$hits" ] && fail "$f: $(echo "$hits" | head -3 | tr '\n' ' ')"
done

# ── 3. no "am I backtesting?" branches ─────────────────────────────────────
heading "3. no code asks whether it is backtesting"
for f in $(non_test_sources "$CLEAN_CRATES"); do
    hits=$(grep -n -i -E 'is_backtest|in_backtest|backtest_mode|is_live_mode' "$f")
    [ -n "$hits" ] && fail "$f: $(echo "$hits" | head -3 | tr '\n' ' ')"
done

# ── 4. crates only depend downward ─────────────────────────────────────────
# nsc-core ← nsc-ta ← nsc-strategy ← the drivers. Never the other way.
heading "4. the clean crates never reach down into the messy ones"
core_deps=$(manifest_body crates/nsc-core/Cargo.toml | grep -o -E '^nsc-[a-z]+')
[ -n "$core_deps" ] && fail "nsc-core depends on other nsc crates: $(echo "$core_deps" | tr '\n' ' ')"

ta_deps=$(manifest_body crates/nsc-ta/Cargo.toml | grep -o -E '^nsc-[a-z]+' | grep -v '^nsc-core$')
[ -n "$ta_deps" ] && fail "nsc-ta may only depend on nsc-core, but also has: $(echo "$ta_deps" | tr '\n' ' ')"

strat_deps=$(manifest_body crates/nsc-strategy/Cargo.toml | grep -o -E '^nsc-[a-z]+' \
    | grep -v -E '^nsc-(core|ta)$')
[ -n "$strat_deps" ] && fail "nsc-strategy may only depend on nsc-core and nsc-ta, but also has: $(echo "$strat_deps" | tr '\n' ' ')"

# ── 5. libraries never panic ───────────────────────────────────────────────
heading "5. no unwrap, expect, panic!, todo! or unimplemented! in libraries"
for f in $(non_test_sources "$LIB_CRATES"); do
    # Strip line comments first: a doc comment explaining why something does
    # not unwrap should not be reported as an unwrap.
    hits=$(sed 's|//.*||' "$f" | grep -n -E '\.unwrap\(\)|\.expect\(|panic!|todo!|unimplemented!')
    [ -n "$hits" ] && fail "$f: $(echo "$hits" | head -3 | tr '\n' ' ')"
done

# ── 6. no file over the hard limit ─────────────────────────────────────────
heading "6. no .rs file over 250 lines"
for f in $(find crates -name '*.rs'); do
    n=$(wc -l < "$f" | tr -d ' ')
    [ "$n" -gt 250 ] && fail "$f is $n lines (hard limit 250)"
done

# ── 7. mod.rs is a front door, not a room ──────────────────────────────────
heading "7. mod.rs holds docs and declarations only"
for f in $(find crates -name 'mod.rs'); do
    hits=$(sed 's|//.*||' "$f" | grep -n -E '^[[:space:]]*(pub[^ ]* )?(struct|enum|fn|impl|trait)[[:space:]]')
    [ -n "$hits" ] && fail "$f has code in it: $(echo "$hits" | head -3 | tr '\n' ' ')"
done

# ── 8. no pip numbers outside config/ ──────────────────────────────────────
heading "8. distances are measured in ATR, and pip numbers live in config/"
for f in $(non_test_sources crates); do
    # Case-insensitive, because STOP_PIPS is as much a pip number as stop_pips.
    # The [^=]* allows a type annotation to sit between the name and the value.
    hits=$(sed 's|//.*||' "$f" \
        | grep -n -i -E 'Pips::(new|from)[^)]*[0-9]|[a-z_]*pips[a-z_]*[^=]*=[[:space:]]*-?[0-9]')
    [ -n "$hits" ] && fail "$f has a pip number in code: $(echo "$hits" | head -3 | tr '\n' ' ')"
done

# ── 9. no secrets in git ───────────────────────────────────────────────────
heading "9. no secrets tracked by git"
if command -v git > /dev/null 2>&1; then
    secrets=$(git ls-files | grep -E '(^|/)\.env$|\.pem$|(^|/)\.env\.[a-z]+$' | grep -v '\.env\.example$')
    [ -n "$secrets" ] && fail "these are committed: $(echo "$secrets" | tr '\n' ' ')"
fi

# ── result ─────────────────────────────────────────────────────────────────
if [ "$FAILED" -eq 0 ]; then
    printf '\nAll nine checks passed.\n'
else
    printf '\nSomething above needs fixing. ci/README.txt says what each check is for.\n'
fi
exit "$FAILED"
