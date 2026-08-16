# 🟡 It only runs from the project root

**Found** 14 August 2026.
**Where** every path in the project — `config/`, `assets/card/`, `preview/`.

## What he sees

```sh
cd ~
~/Desktop/TRADES-BY-NSC/nsc_trades/target/debug/nsc-work-man
```

It starts, and then cannot find a single one of his levels. Run it from the
project folder and it works.

## Why

Every path is written relative to wherever the program was started:
`config/pairs`, `assets/card/heartbeat.html`, `preview/status.png`. Start it
somewhere else and all of them point at nothing.

## What it costs

Nothing today — it is always started from the project folder, and the failure
is loud rather than quiet. It says it cannot read the file and stops.

It matters at deploy: a service file, a container or a cron job all start a
program from wherever they please.

## The fix

Work out the folder once at startup and hand it down, rather than each file
guessing. Either from the binary's own location or from an environment
variable set alongside the token.

**Not before hosting.** Doing it now adds an argument to every path in the
project to solve a problem nobody has.
