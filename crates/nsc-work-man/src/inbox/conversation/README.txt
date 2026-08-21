WORKING OUT WHAT HE MEANT
=========================

Everything he sends the bot lands here: a command, a button he tapped, or a
line of prices.

Telegram has no idea what a conversation is. Every message arrives on its own
with nothing attached saying what came before it. So "1.28" is meaningless on
its own -- it only means something if we remember that he picked GBPUSD and
then picked Weekly. That memory lives in this folder and nowhere else.


THE FILES

  mod.rs        The front door. What is inside and what the rest of the
                inbox can see.

  adding.rs     Where he is. Which pair, which chart, and which flow he is
                part-way through. One struct, and it is the only state the
                bot keeps about him.

  route.rs      What a message means, given where he is. Reads top to
                bottom: the first thing that matches wins.

  saving.rs     A line of prices, saved, and said back to him -- what the
                pair now holds, and a picture of where it landed.

  reading.rs    Pulling the numbers out of a message. One per line, six on
                one line, or one on its own.


THE ORDER IN route.rs IS THE BEHAVIOUR

  Close comes first, above everything. Wherever he is, backing out means the
  same thing. Put it lower and it would be swallowed by whichever flow he
  happened to be in.

  Then the questions that need no memory -- /help, /status.

  Then the flows that own their own conversations: stopping a pair, and one
  pair's page.

  Then /level, Undo, and picking a pair or a chart.

  Prices come LAST, because a price is what a message is when it is nothing
  else. Read them earlier and "4-hour" would be a number before it was a
  chart.


WHY IT IS A FOLDER

  It was one file and it went past 250 lines, which is the limit in
  CLAUDE.md. Past that you scroll a file rather than read it, and the part
  you need is never on screen with the part it depends on.

  The bug that made this urgent was exactly that kind: `dropping` was set in
  one place and read 100 lines away, and nothing cleared it in between.


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
