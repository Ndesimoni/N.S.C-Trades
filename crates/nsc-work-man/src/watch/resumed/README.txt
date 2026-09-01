WHAT TO SAY WHEN THE OPENING HOURS ARE OVER
===========================================

  One report per pair, per session: every zone price is ALREADY SITTING IN.

  Marked "already at", never "just arrived". Price can walk into a zone while
  the bot is quiet -- over a weekend, or through the opening hours -- and still
  be there when it starts looking. The watcher only fires on a CHANGE, so
  nothing would ever be said about it. And saying "arrived" would stamp a move
  made on Sunday with Monday's time.


THE FILES

  mod.rs     The front door, and why it waits.
  awake.rs   The report, and what it remembers.
  tests.rs   What it remembers -- which is the whole of the behaviour.


IT REMEMBERS TWO THINGS, AND BOTH WERE BUGS

  WHICH SESSION. It was a plain "have I greeted?" flag, set once and never
  cleared. A bot left running from Friday greeted on the Friday and then never
  again -- so the Sunday open, after two days of silence, was exactly when the
  report was worth most and exactly when it would not come.

  WHICH PAIR. It was one flag for the whole bot. He sends a level mid-session,
  its bands are built fresh, and its Watch starts over -- so the first price is
  only a baseline and produces no arrival. The session was already greeted, so
  nothing said price was sitting in the zone he had just drawn. He got "your
  levels are live" and then silence.

  He usually draws a level BECAUSE price is near it, so that was the common
  case, not the corner one.

  reload.rs now says which pairs it built fresh, and run.rs calls forget() on
  each. Only those are re-reported, so the pairs he did not touch are not
  announced to him twice.


WHY IT WAITS FOR THE SETTLE WINDOW

  It used to go the moment the socket opened. The first hours of a day are
  where a move gets faked and taken back, so that report described a position
  price had not committed to -- and he would either act on it or learn to
  ignore it. Both are bad.

  It waits for settle_hours in config/when.toml, then says where things
  actually stand.

  NOT gated on "may a trade be suggested". Friday settles four hours in like
  any day and still opens no trade; gating on the trade would silence this
  every Friday.


THE OPENING HOURS ARE WATCHED, NOT SPOKEN ABOUT
===============================================

  settle_hours in config/when.toml is 4. For those four hours after a session
  opens, the bot says nothing at all -- no approach, no in-the-zone, no candle
  close.

  It is not asleep. Prices come down the line and are checked against the
  bands exactly as always, so the watcher keeps its record of where price is and
  which zones it is sitting in. Only the sending is held.

  WHY. The first hours of a day are where a move gets faked and taken back. A
  zone touched at the open and abandoned twenty minutes later is a buzz he has
  to ignore, and a buzz he learns to ignore is one that costs him the alert
  that mattered.

  WHAT ARRIVES WHEN THEY END. One report per zone price is actually sitting
  in, marked "already at" rather than "just arrived". That distinction is the
  whole reason resumed/ exists -- saying "arrived" would put a move made at
  the open onto the clock of the moment the window closed.

  THE GREETING IS PER SESSION AND PER PAIR, NOT ONE FLAG FOR THE BOT. It used
  to be set once and never cleared, which cost twice: a bot left running from
  Friday greeted once and never again, and a level he sent mid-session got
  "your levels are live" and then silence about the zone price was already
  sitting in. See resumed/README.txt.

  SETTLED IS NOT THE SAME AS TRADEABLE. Friday settles four hours in like any
  other day and still opens no trade. Gating the report on "may a trade be
  suggested" would silence it every Friday.


WHAT TUESDAY SAYS

  Price can walk into a zone during Monday's silence and still be there when
  Tuesday opens. The watcher fires on a CHANGE, so nothing would ever be said
  about it.

  So the first thing after silence is a report of what was FOUND. The card
  says "already in the zone", not "arrived" — because nobody watched it happen
  and an arrival card would put a Monday move on a Tuesday clock.
