inbox/ — the bot listening
==========================


WHAT THIS FOLDER IS FOR

  The other side of Telegram. telegram/ talks; this listens.

  He sends a level from his phone and it lands in config/pairs/. Then it
  draws the pair and sends the picture back, so he can see where the band
  actually sat.


A REFUSED MESSAGE IS NOT A SENT MESSAGE

  Telegram answers 200 with `ok: false` when it will not take a message. That
  was printed to a terminal he is not watching, and `say` returned Ok — so
  everything upstream believed he had been replied to.

  He would have seen nothing, and had no way to tell that from a dead bot.

  It is an error now. And the text of what went wrong is ESCAPED before it
  goes in, because every message here is parsed as HTML: a stray `<` in an
  error is an unclosed tag, and Telegram refuses the whole message. The reply
  that says what went wrong is exactly the one that has to arrive.


/status ALWAYS ANSWERS

  It used to reply "Could not do that" when the card would not draw, which is
  the single most misleading thing it could say -- the whole job of this
  command is telling him the bot is alive.

  The words carry the answer; the picture only carries it better. If the card
  fails, the words go on their own.

  And on a day nothing is watched it says so in a sentence rather than drawing
  a card of dashes. The card's useful column is how far price is from the
  nearest zone, and on a quiet day no price has arrived, so every row would be
  blank.


HOW /status ANSWERS

  The WATCHER holds the live picture — which bands are sized, where price was
  last, which zones it is sitting in. The inbox runs beside it and has none of
  that.

  So the watcher PUBLISHES A COPY whenever it changes, and the inbox reads the
  latest one. Nothing is shared and nothing is locked: the reader gets whatever
  the last published copy was, which for "is it running and what is close" is
  exactly right.

  The copy is taken before drawing, not held across it. Holding the borrow
  while Chrome runs would stop the watcher publishing for a second or two.


IT RUNS INSIDE THE BOT

  Spawned beside the watcher, so `cargo run -p nsc-work-man` is the whole
  thing.

  It was a second program for a while. That meant two terminals and
  remembering both — and if it was not up, a level he sent went NOWHERE and
  nothing said so. He would find out days later when it never fired.

  The two do not talk to each other. The inbox writes a pair's file and the
  watcher notices the folder changed, which is how it already worked.

  IT CANNOT STOP. If this task ends, levels go nowhere again — so it catches
  its own trouble, waits fifteen seconds and listens again.


ONE PAIR, AND WHAT HE CAN DO TO IT

      /pairs      ->  every pair he has
      tap GBPUSD  ->  what it holds, and three things:
                      [+ Add levels] [− Take one off] [✗ Stop watching]

  THIS IS WHERE A LEVEL GETS TAKEN OFF. Undo only ever reached what the last
  message added — which covers a typo the moment it happens, and does nothing
  at all for "that 1.15 from last week was wrong". That was the gap.

  The levels come back as buttons, one each, written exactly as they are in the
  file. Tapping one hands back what is there rather than anything having to be
  guessed at.

  The price is matched AS A NUMBER, the same way saving refuses a duplicate. He
  may tap 1.15000 having typed 1.15, and as text those are two different
  levels.

  The comments in the file survive it. He is meant to be able to open one and
  read it.


PUTTING ONE BACK

      /restore  ->  which one?  ->  [pair]  ->  back

  ONE TAP. It takes nothing away, so there is nothing to be careful about.

  It comes back under the PAIR'S OWN NAME, whatever the file is called — the
  name on disk is bookkeeping, the name inside the file is the pair. Restoring
  GBPUSD-2 lands as GBPUSD.

  AND IT REFUSES TO LAND ON A PAIR HE IS ALREADY WATCHING. He may have stopped
  a pair, drawn it again from scratch, and then reached for the old set —
  which would replace the levels he is using with the ones he put aside, and
  say nothing. It says so instead, and leaves both alone.


STOPPING A PAIR

      /remove   ->  which pair?  ->  [pair]  ->  stop it? / keep it

  TWO TAPS, NOT ONE. It throws away every level he has drawn for that pair —
  months of chart work — and the first tap is made on a phone while he is
  doing something else. The second one tells him how many levels are on it
  before he confirms.

  AND IT IS MOVED, NOT DELETED. The file goes to config/pairs/removed/ and
  comes back by being moved out again. The reply says where it went.

  Retiring the same pair twice does not overwrite the first set. He may add it
  back, draw it again and drop it again, and the first set is still the one he
  spent an evening on.

  The watcher notices within ten minutes, on its own, and stops watching it.


THE FILES

  mod.rs        The front door -- what is inside and what the rest of the
                bot can see.

  hearing.rs    The long poll that never gives up, and reading what Telegram
                answers. See ONLY ONE COPY AT A TIME below.

  conversation/ Working out what he meant, and what to say back. Its own
                folder, with its own README. The
                flow that ADDS levels lives here.

  asked.rs      /help and /status — the two things he asks outright, plus
                registering the commands with Telegram so they appear in the
                tap-list beside the message box.

  one.rs        One pair's page — what it holds, and what he can do to it.

  pairs.rs      Stopping a pair, and putting one back.

  picture.rs    Drawing where the levels landed.

  dropping.rs   Taking one level off a pair. Each level goes up as its own
                button reading "weekly 1.21279", and the chart name on it is
                checked -- see BACKING OUT below for why.

  talking.rs    Saying it, with buttons. It is also the one place that adds
                the Close row, and it escapes anything going into a message,
                because they are parsed as HTML.

  README.txt    This file.


HOW IT GOES

  /level              which pair?        [XAUUSD] [GBPUSD] [+ new pair]
  XAUUSD              which timeframe?   [Weekly] [Daily] [4-hour]
  Weekly              send prices
  4520 4000 3800      saved, and here is the chart          [↩ Undo]

  The pair and the timeframe STICK. Six weekly levels is two taps and six
  numbers.


FOUR THINGS THAT ARE DELIBERATE

  1. THE BUTTONS ARE A REFUSED MESSAGE IS NOT A SENT MESSAGE

  Telegram answers 200 with `ok: false` when it will not take a message. That
  was printed to a terminal he is not watching, and `say` returned Ok — so
  everything upstream believed he had been replied to.

  He would have seen nothing, and had no way to tell that from a dead bot.

  It is an error now. And the text of what went wrong is ESCAPED before it
  goes in, because every message here is parsed as HTML: a stray `<` in an
  error is an unclosed tag, and Telegram refuses the whole message. The reply
  that says what went wrong is exactly the one that has to arrive.


/status ALWAYS ANSWERS

  It used to reply "Could not do that" when the card would not draw, which is
  the single most misleading thing it could say -- the whole job of this
  command is telling him the bot is alive.

  The words carry the answer; the picture only carries it better. If the card
  fails, the words go on their own.

  And on a day nothing is watched it says so in a sentence rather than drawing
  a card of dashes. The card's useful column is how far price is from the
  nearest zone, and on a quiet day no price has arrived, so every row would be
  blank.


HOW /status ANSWERS

  The WATCHER holds the live picture — which bands are sized, where price was
  last, which zones it is sitting in. The inbox runs beside it and has none of
  that.

  So the watcher PUBLISHES A COPY whenever it changes, and the inbox reads the
  latest one. Nothing is shared and nothing is locked: the reader gets whatever
  the last published copy was, which for "is it running and what is close" is
  exactly right.

  The copy is taken before drawing, not held across it. Holding the borrow
  while Chrome runs would stop the watcher publishing for a second or two.


IT RUNS INSIDE THE BOT

  Spawned beside the watcher, so `cargo run -p nsc-work-man` is the whole
  thing.

  It was a second program for a while. That meant two terminals and
  remembering both — and if it was not up, a level he sent went NOWHERE and
  nothing said so. He would find out days later when it never fired.

  The two do not talk to each other. The inbox writes a pair's file and the
  watcher notices the folder changed, which is how it already worked.

  IT CANNOT STOP. If this task ends, levels go nowhere again — so it catches
  its own trouble, waits fifteen seconds and listens again.


ONE PAIR, AND WHAT HE CAN DO TO IT

      /pairs      ->  every pair he has
      tap GBPUSD  ->  what it holds, and three things:
                      [+ Add levels] [− Take one off] [✗ Stop watching]

  THIS IS WHERE A LEVEL GETS TAKEN OFF. Undo only ever reached what the last
  message added — which covers a typo the moment it happens, and does nothing
  at all for "that 1.15 from last week was wrong". That was the gap.

  The levels come back as buttons, one each, written exactly as they are in the
  file. Tapping one hands back what is there rather than anything having to be
  guessed at.

  The price is matched AS A NUMBER, the same way saving refuses a duplicate. He
  may tap 1.15000 having typed 1.15, and as text those are two different
  levels.

  The comments in the file survive it. He is meant to be able to open one and
  read it.


PUTTING ONE BACK

      /restore  ->  which one?  ->  [pair]  ->  back

  ONE TAP. It takes nothing away, so there is nothing to be careful about.

  It comes back under the PAIR'S OWN NAME, whatever the file is called — the
  name on disk is bookkeeping, the name inside the file is the pair. Restoring
  GBPUSD-2 lands as GBPUSD.

  AND IT REFUSES TO LAND ON A PAIR HE IS ALREADY WATCHING. He may have stopped
  a pair, drawn it again from scratch, and then reached for the old set —
  which would replace the levels he is using with the ones he put aside, and
  say nothing. It says so instead, and leaves both alone.


STOPPING A PAIR

      /remove   ->  which pair?  ->  [pair]  ->  stop it? / keep it

  TWO TAPS, NOT ONE. It throws away every level he has drawn for that pair —
  months of chart work — and the first tap is made on a phone while he is
  doing something else. The second one tells him how many levels are on it
  before he confirms.

  AND IT IS MOVED, NOT DELETED. The file goes to config/pairs/removed/ and
  comes back by being moved out again. The reply says where it went.

  Retiring the same pair twice does not overwrite the first set. He may add it
  back, draw it again and drop it again, and the first set is still the one he
  spent an evening on.

  The watcher notices within ten minutes, on its own, and stops watching it.


THE FILES.

     Whatever is in config/pairs/ becomes a button. Not a list in this code —
     that was the mistake the old settings.rs made, and two lists always
     disagree in
     the end. A pair exists because its file exists.

  2. IT ONLY OBEYS HIM.

     Every message is checked against his Telegram user id, and anything else
     is ignored WITHOUT A WORD. A bot that replies "you are not allowed" is a
     bot that just told a stranger it exists.

     This is also why levels come from the private chat and not the channel:
     CHANNEL POSTS CARRY NO SENDER AT ALL. Telegram strips it, because a post
     is from the channel rather than from a person. The private chat is the
     only place the bot can tell who is talking.

  3. IT SHOWS RATHER THAN ASKS.

     It does not ask "is this right?" before saving. It saves, draws the pair,
     and sends the picture with an Undo button.

     Asking first only checks he can read his own typing back. The mistake
     that matters is a level in the wrong PLACE, and that is visible in a
     glance on a chart and invisible in a list of numbers.

  4. THE PICTURE GOES TO THE PRIVATE CHAT, NOT THE CHANNEL.

     It is him working, not a signal. Mixing the two turns the signal channel
     into a scratchpad.


WHAT IS NOT HERE YET

  Removing one particular level. Undo takes off what the LAST message added,
  which covers a typo but not "that 3800 from last week was wrong". For that
  he opens the file.


BACKING OUT

  Every keyboard carries a Close button on its own row. It is added in
  talking.rs, in the one function that builds a keyboard, rather than at each
  place that puts buttons up.

  Doing it at each call site would work until somebody added the eleventh
  keyboard and forgot -- and the one without a Close is exactly the flow he
  gets stuck in, buttons covering his own keyboard, on a phone.

  The tap is caught at the very top of conversation/route.rs, before /help,
  before
  the pair pages and before the stop-watching confirmation. Anywhere he can
  be, Close means the same thing.

  It resets what the bot remembered and takes the buttons away. It undoes
  nothing. He asked to be left alone, not to have his levels changed.


A BUTTON THAT IS NO LONGER TRUE

  Telegram keeps old keyboards tappable forever. He can scroll up a week and
  press a button from a conversation that is long finished, and it arrives
  looking exactly like one he pressed a second ago.

  Two things guard against that, and both were bugs first:

  Moving to a different pair forgets the page he was on. `chosen` and
  `dropping` used to survive it, so a level button from an older message took
  its price off whichever pair he was last LOOKING at rather than the one he
  was adding to.

  A level button carries its chart name -- "weekly 1.21279", not "1.21279".
  Reading the last number off any message meant that while the take-one-off
  list was up, sending "1.28 1.31" -- which is how he is TOLD to add two
  levels -- was read as "take 1.31 off", against whichever pair's page he was
  last on.

  Taking a level off says whether it was there. A price that is not on the
  pair changes nothing, which is right. But the reply said "1.28 taken off"
  either way, so a stale tap looked exactly like one that worked.


ONLY ONE COPY AT A TIME

  Telegram hands each message to whichever copy of the bot asks for it first.
  Two running at once split his messages between them at random, and each one
  looks like it is ignoring him.

  Telegram does say so -- it refuses the poll with error 409, "Conflict:
  terminated by other getUpdates request". But only `result` was ever read out
  of the answer, and a refusal carries no `result`, so a refused poll looked
  exactly like a quiet minute. The second copy span on silently, forever,
  while he sent messages nothing was reading.

  It now says which thing is wrong, every fifteen seconds, until one is shut
  down. Same for any other refusal -- a bad token reads the same way and is a
  very different evening.


/status ON A QUIET DAY DRAWS NOTHING

  It sends words and no picture.

  The card's one useful column is how far price is from the nearest zone, and
  on a quiet day no price has arrived to measure from -- every row would read
  as a dash. It also saves running Chrome for the best part of ten seconds to
  say nothing.
