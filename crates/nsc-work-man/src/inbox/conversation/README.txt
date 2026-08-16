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
