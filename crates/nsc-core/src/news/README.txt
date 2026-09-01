news/ — what is coming up on the economic calendar
==================================================


WHAT THIS FOLDER IS FOR

  Knowing that a level is sitting in front of a rate decision rather than in
  front of a quiet Thursday.

  IT DESCRIBES AND IT DECIDES ONE THING: whether an event is worth saying
  something about yet. It does not fetch, it does not draw, and it does not
  read the clock.


THE FILES

  mod.rs      The front door.
  impact.rs   Impact -- high, medium, low, holiday, and unknown.
  event.rs    Event -- one entry, and the name that stops it being said twice.
  due.rs      THE WINDOW, and the grouping of a release onto one card.
  span.rs     Today, or the whole week -- what /news asks for.
  away.rs     How long until it prints, in words: "in 45m", "in 3d 10h".
  rules.rs    The settings, read out of config/news.toml.
  tests/      Thirty-six tests.
  README.txt  This file.


NOTHING HERE READS THE CLOCK OR REACHES ANYTHING

  Every function is handed `now`, exactly like when/. Downloading the week's
  file happens in nsc-data::news, because this crate has no reqwest in its
  Cargo.toml and so it CANNOT reach anything.

  That is what lets the same judgement run in a backtest one day.


TWO WARNINGS FOR ONE RELEASE

  warn_at_minutes is a list -- [5, 1]. A heads-up five minutes out and a last
  call one minute out. His ask, 1 September 2026.

  It was [30, 5] for a few hours and thirty was too early: a warning that far
  out is read, filed and forgotten before the number prints.

  ONE MARK IS LIVE AT A TIME. The 5 owns from five minutes out down to one
  minute out; then the 1 takes over and runs on past the release.

      5  |--------|                   at-5 up to at-1
      1           |-------------|     at-1 through at+5

  They never overlap, and that is the point. Windows that both stayed open
  would make the second card either impossible to tell from the first or
  impossible to send at all. due_at() says WHICH one is live, and the watcher
  remembers each separately -- keyed on the event alone, the half-hour card
  would silence the five-minute one.


THE WINDOW HAS TWO EDGES AND THE FAR ONE IS THE IMPORTANT ONE

  The near edge is obvious: say something before it prints.

  The far edge is what makes a restart survivable. The bot comes back at two
  in the afternoon and the week's file is full of this morning. Without a far
  edge every one of those reads as "coming up", and they all arrive at once.

  So an event speaks between the widest warn_at_minutes before it and
  stale_minutes after, and is silent on either side of that.

  The far edge belongs to the LAST mark, so a restart just after a number
  prints gets the one-minute card rather than one headed "in 5 minutes" about
  something that already happened.

  It is the same lesson the zone alert learned when it separated "is in the
  zone" from "was already in the zone" -- a Monday move must never get a
  Tuesday clock.


AN UNKNOWN RATING IS SILENT, NOT LOUD

  The feed writes the rating as a word. A word this code has not been taught
  becomes Impact::Unknown, which matches nothing in the settings and therefore
  never earns a message.

  That is the safe direction. Guessing would put an unrecognised rating on his
  phone looking exactly like a rate decision.


ONE CARD PER RELEASE, NOT ONE PER LINE

  Three Australian CPI numbers print in the same second. He reads that as one
  release, so `together` puts everything sharing a timestamp on one card.

  Sent separately it would buzz his phone three times, and the whole design
  rests on messages being rare enough that he still opens them.


NOTHING IS LEFT OUT OF A LIST, IT IS MARKED

  Both /news lists carry the whole span -- today keeps the morning's releases
  and the week keeps Monday's.

  A week with its first three days silently missing does not read as a week.
  It reads as a quiet one, and that is the wrong answer to the question he
  asked. So every row says which side of now it is on: PASSED, or how long it
  has to go.

  Same lesson as the zone alert separating "is in the zone" from "was already
  in the zone" -- leaving something out and marking it are different answers,
  and only one of them is true.


THE UNITS SHRINK AS IT GETS CLOSER

  "in 45m", then "in 3h 20m", then "in 3d 10h".

  Two days out the minutes do not matter. Forty minutes out they are the only
  thing that does -- and "in 0h" for something forty minutes away reads as a
  card that failed to fill in rather than as a number.
