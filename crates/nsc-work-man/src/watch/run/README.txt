run/ — starting the bot up, and keeping it up
=============================================


WHAT THIS FOLDER IS FOR

  The supervision loop. Start here — everything else in watch/ is called from
  it.

  IT IS MEANT TO RUN FOR WEEKS. Everything is shaped by that: the line WILL
  drop, and dropping must not be the end of it.


THE FILES

  mod.rs       The front door.

  forever.rs   The loop. Load the settings, open the line to TWS, watch it,
               and open it again when it ends.

  armed.rs     Reading the levels again when he sends one, telling him they
               are being watched, and the same on demand for --bin cards.

  picture.rs   The live picture, for anything that ASKS rather than watches —
               /status and the morning heartbeat.

  README.txt   This file.


WHAT THE LOOP DOES

    is today quiet, or is there nothing to watch?
      -> the heartbeat still goes out, levels are still picked up, sleep a
         minute. NO LINE IS OPENED AT ALL.

    otherwise
      -> hold the price line open until it ends

    it ended cleanly            he sent a level, or the session closed
    it ended badly              tell trouble.rs, and OPEN A FRESH LINE TO TWS


WHY A BROKEN LINE REOPENS THE WHOLE CONNECTION

  Because the connection itself may be what died.

  TWS restarting, or the Mac going to sleep, leaves a Client that will refuse
  every subscription from now on -- and subscribing again on a dead one fails
  identically, forever. Nothing recovers until a fresh line is opened.

  FAILING TO RECONNECT KEEPS THE OLD LINE RATHER THAN STOPPING. The gateway
  may be halfway through starting up, and the next pass is thirty seconds
  away.


FAILING AT STARTUP IS FATAL, ON PURPOSE

  Unlike a web API, this feed needs a program logged in. A bot that starts up
  and watches nothing is worse than one that says why it did not start --
  because from his phone, both of them are silence.


THE INBOX RUNS BESIDE THE WATCHER

  Spawned here, so one command is the whole bot. It was a second program for a
  while, which meant two terminals and remembering both -- and if it was not
  up, a level he sent went nowhere and nothing said so.

  THEY DO NOT TALK TO EACH OTHER ABOUT LEVELS. The inbox writes a file and the
  watcher notices it changed.

  But /status has to answer from the LIVE picture -- which bands are sized,
  where price was last -- so the watcher publishes a copy of that (picture.rs)
  and the inbox reads the latest.


WHAT MUST NEVER STOP THE BOT
============================

  It is meant to run for weeks. Three things used to end it, or end the
  socket, and none of them were the price line actually breaking.

  SIZING A PAIR'S BANDS. reload.rs asks the feed for history to work out how
  thick a band should be. That answer used to travel out of run() and stop
  the whole bot -- he sends a level from his phone, the feed is slow for
  ten seconds, and the bot says "stopped" and quits. It now keeps whatever
  bands that pair already had and tries again on the next look.

  FETCHING A CANDLE. closes/ asks for the newest candle when price is at a
  zone. That is a REST request on a completely different connection from the
  price websocket, and its failure used to drop the line. Repeated, it told
  him the price line was down while the price line was perfectly fine.

  HAVING NOTHING TO WATCH. Removing the last pair left it subscribing to no
  symbols at all. Nought refused out of nought asked read as every pair being
  refused, so it reported the line as broken every thirty seconds. It now
  waits quietly, the same as it does at the weekend, and picks the levels up
  when he sends some.

  SAYING THE LEVELS ARE ARMED. That used `?`, so Chrome refusing to start --
  because his own browser held the profile -- killed the bot at startup, on
  the one message whose only job is to say "your levels are live". They ARE
  live either way.

  The rule under all four: A THING THAT FAILS TO BE SAID IS NOT THE PRICE LINE
  BREAKING, and a card that will not draw is not a reason to stop. Only the
  price line itself going down is that.



A LEVEL SENT WHILE IT IS RUNNING IS PICKED UP

  The levels used to be read ONCE, at startup. He would send one from his
  phone, the inbox would save it correctly, the file would be right — and the
  watcher would never look again. Nothing said so. The level simply did
  nothing until the next restart, which might be days.

  Now the folder is checked every ten minutes, BY THE CLOCK ON THE FILES.
  Parsing every pair file to find out that nothing happened is work done for
  nothing, and nothing is the normal answer. The count is checked too — a file
  deleted leaves every remaining timestamp exactly as it was.

  A PAIR WHOSE LEVELS ARE UNTOUCHED KEEPS THE WATCH IT ALREADY HAD. Rebuilt,
  it would forget which zones price is sitting in and announce every one of
  them again as though it had just arrived. Only the changed pair costs a
  request.

  The line is then closed and opened again, because THE SUBSCRIPTION IS FIXED
  WHEN THE SOCKET OPENS — a pair added to a live one would never be asked
  about. No thirty-second pause on that path; he is standing there having just
  sent it.

  IT ALSO HAPPENS ON QUIET DAYS. The weekend is exactly when he does his chart
  work, and the check lived inside the socket loop at first — which does not
  run on a quiet day. A level sent on Sunday sat unarmed until Tuesday.

  He gets a card back: a tick, "Got it. Your levels are live", and the count
  of what is being watched.

  NO PAIR NAMES. He has just sent it — he knows what he sent, and
  the inbox has already sent back a picture of where the bands landed, with
  the pair on it in his colours. Repeating that is a second message telling
  him something he already had.

  What that picture cannot say is that they are being WATCHED. Saved and armed
  were two separate states and nothing told him which one he had. That is the
  whole job of this card.

  The count is the one detail worth having: he sees the number went up.
