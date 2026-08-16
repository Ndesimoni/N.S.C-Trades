# 🟡 It only runs from the project root

**Where** every path — `config/`, `assets/card/`, `preview/`
**Found** 14 Aug 2026

## What happens

Started from anywhere else it finds no levels, no templates, nothing. Every
path is relative to where the program was started.

Loud, not quiet — it says it cannot read the file and stops.

## Fix

Work the folder out once at startup and hand it down. From the binary's own
location, or an environment variable set beside the token.

Not before hosting.
