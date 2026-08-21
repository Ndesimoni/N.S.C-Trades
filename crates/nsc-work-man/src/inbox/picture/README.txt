picture/ — sending him a picture of a pair
==========================================


WHAT THIS FOLDER IS FOR

  Getting a chart onto his phone.

  Reading a price back at him only proves he can read his own typing. A picture
  shows the PLACE, which is the thing that actually goes wrong -- and it is how
  he reads a chart anyway.


TWO DIFFERENT QUESTIONS, AND THEY READ DIFFERENTLY

  landed.rs answers "did that land where I drew it?" -- the reply to having
  just saved a level. Always the weekly, because it shows every level a pair
  has and his are years apart.

  asked.rs answers "let me look at this pair" -- him asking, on whichever of
  his three charts he picked, with nothing saved to bring it about.

  THE WORDING WHEN IT FAILS IS THE REASON THEY ARE APART. landed.rs says "the
  levels are safe", because a picture failing right after a save looks exactly
  like the save failing. asked.rs must not say that: nothing was saved, and
  telling him his levels are safe would be answering a question he never asked.


THE FILES

  mod.rs      The front door. What is inside, and what the inbox can call.

  landed.rs   show -- where the levels he just sent landed.

  asked.rs    of_pair -- a chart he asked for, and the caption that goes on it.

  sending.rs  The two steps both share: drawing it, and getting it to him.

  tests.rs    What the caption says about a chart holding none of his levels.

  README.txt  This file.


THE CAPTION SAYS WHEN NOTHING REACHED

  150 four-hour candles is about twenty-five days. A weekly level drawn two
  years ago is simply off the top or bottom of that, so the chart draws
  perfectly and comes out empty.

  A chart that is correctly empty and a chart whose bands failed to draw are
  the same picture otherwise, and he would report the second one as a bug.

  So the caption counts. All of them on it says nothing extra -- that is the
  ordinary answer on a weekly and does not need remarking on. None of them says
  so and points him back at the weekly.

  A PAIR WITH NO LEVELS AT ALL GETS A PLAIN CHART. Nought on it out of nought
  is every level he has, not none of them, and it used to tell him that none of
  his 0 levels had reached.


IT GOES TO THE PRIVATE CHAT, NOT THE CHANNEL

  Both of them. This is him working, not a signal, and mixing the two turns the
  signal channel into a scratchpad.


ASKING FOR A CHART

      /chart  ->  which pair?  ->  [pair]  ->  which chart?  ->  picture
      /pairs  ->  [pair]  ->  [📈 Chart]  ->  which chart?  ->  picture

  TWO DOORS INTO THE SAME PLACE. /chart is for when he already knows which pair
  he wants. The button is for when he is on the pair's page anyway, and it sits
  next to the list of levels it is about to draw.

  IT USED TO BE REACHABLE ONLY BY SAVING A LEVEL. The picture went out as the
  reply to "here is where your levels landed", so seeing a chart meant adding
  something in order to see one. bin/levels.rs could draw one, and that is on
  his Mac rather than his phone.

  THE THREE TIMEFRAME BUTTONS ARE SHARED WITH ADDING A LEVEL, and a button only
  sends its own word back -- "Weekly" arrives identical either way. `chart_of`
  on Adding is the only thing that says which question he is answering, and it
  is checked BEFORE the adding flow's timeframe for exactly that reason.

  It is cleared the moment the chart goes, so his next "Weekly" is a level
  again. And it is cleared when he moves to a different pair: Telegram keeps old
  keyboards tappable forever, so a chart question left hanging on the last pair
  would turn a tap made a week later into a picture of the wrong one.

  THE WEEKLY IS THE ONE THAT SHOWS EVERYTHING. His levels are years apart. The
  daily and the 4-hour are for reading one level closely, and they will often
  hold none of them at all -- 150 four-hour candles is about twenty-five days.

  So the caption says when nothing reached. A chart that is correctly empty and
  a chart whose bands failed to draw are otherwise the same picture, and he
  would report the second one as a bug. See review/README.txt.
