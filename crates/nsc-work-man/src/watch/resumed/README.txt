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
