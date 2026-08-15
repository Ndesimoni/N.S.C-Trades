telegram/ — sending
===================


WHAT THIS FOLDER IS FOR

  Getting pictures and words onto his phone. The listening side is in
  bin/inbox/.


THE FILES

  mod.rs      The front door.
  out.rs      The sending.
  error.rs    What can go wrong, and whether another go would help.
  tests.rs    Three tests, all on that second question.
  README.txt  This file.


TWO CHATS, AND THEY ARE NOT THE SAME

  send      -> the channel. Signals and alerts.
  send_to   -> wherever you say. His private chat is where his own working
               goes — a chart he asked for while adding a level is not a
               signal, and mixing the two turns the channel into a scratchpad.


SEVERAL PICTURES GO AS ONE MESSAGE

  Telegram calls it a media group. It buzzes the phone ONCE, but the pictures
  sit apart with a gap and each one opens on its own when tapped — which is
  why the cards are separate files rather than one tall picture.

  The caption goes on the first picture and shows under the whole group. Put
  it on every picture and Telegram repeats it under every picture.


THE TRAP, WHICH CAUGHT US ONCE

  TELEGRAM REFUSES POLITELY. A 200, and ok: false in the body.

  A reply that parses is not a message that arrived. Twelve Data does the same
  thing with a 401 — met twice in one afternoon, so it is a pattern rather
  than bad luck.

  And it says "Too Many Requests" in WORDS rather than a code we can match on,
  so the words are what there is to go by. Anything else — a bad token, a chat
  that does not exist, a caption too long — is settled and will not change.
