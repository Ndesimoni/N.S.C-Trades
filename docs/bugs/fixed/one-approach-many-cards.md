# 🔴 One approach sent card after card — fixed 31 Aug 2026

**Where** `nsc-core::levels::watch::clear_of`
**Found** 31 Aug 2026, by him, watching it happen · **Fixed** the same day

## What he saw

> *"When price is approaching a level and then price goes back, you keep
> sending me a message every time the price comes back to that level... It
> ended up sending so many cards on AUD/USD."*

## What was happening

Two distances decide whether an alert can fire again:

```text
    approaching     reach          how close counts as at the level
    properly gone   CLEAR_BY x band thickness   how far to re-arm
```

**They are in different units, and that is the whole bug.** `reach` comes from
`approach_pips` — a flat 4 pips — while the reset is a share of the band. One
scales with the pair and the other does not, so on some bands they overlap.

His AUD/USD daily level, measured:

```text
    the band, 0.46 of a 49.3-pip daily candle     22.7 pips
      0.713865  ────────────────────────────────  0.716135

    approaching reaches 4.0 pips out    down to   0.713465
    reset sat 2.3 pips out              below     0.713638   <-- INSIDE
```

**Every price between 0.713465 and 0.713638 was both "approaching" and
"properly gone" at once.** So a wobble of under two pips reset the band and
fired the alert again. And again.

Sampling four prices an hour through August 2026 gives **45 alerts on that one
level in one month**. Real ticks arrive about once a second, so the true count
is far higher — which matches the pile of cards he got.

## Why it hid

**On gold the two numbers are nowhere near each other.** A touch reaches ten
cents; the way home is nearly eight dollars. Every test in `watching.rs` was
written on his gold weekly band, where no overlap is possible.

It needed a band thin enough, and a `reach` fixed in pips rather than scaled,
for the two to meet. That is a per-pair condition — which is exactly the shape
of failure this project warns about: *"it works on the pair you tested and
quietly stops working on every other one."*

## The fix

**The way home now starts where approaching ends.**

```rust
price > band.top + reach + gone || price < band.bottom - reach - gone
```

The two can never overlap again, however thin the band or wide the reach. A
visit is over once price has left the region that counts as being at the level
— not once it has left the band, which it never really had.

Two tests pin it, and both were checked to fail with the fix removed.

## Still worth doing

**`approach_pips` is the only distance in this project written in pips.**
Everything else is a share of something, which is why everything else scales.
This fix makes the overlap impossible regardless — but the underlying oddity is
still there, and it is what made the overlap possible in the first place.
