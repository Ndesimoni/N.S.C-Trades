review/ — drawing a pair's levels
=================================


WHAT THIS FOLDER IS FOR

  Turning a pair's levels into a picture of them, on whichever chart he asked
  for.

  This is the check that matters. Everything in this project is measured
  against his levels, so the first question is always whether the band the code
  builds sits where the one he drew sits.

  Reading a price back at him only proves he can read his own typing. A picture
  shows the PLACE, which is the thing that actually goes wrong.


THE FILES

  mod.rs      The front door. What is inside, and what the rest of the bot can
              see.

  drawn.rs    Drawn — what came back, and how many of his levels are actually
              on it. Also on_the_chart, which works that count out.

  picture.rs  Fetching the candles and drawing them. picture_of is the one
              thing outside this folder calls.

  tests.rs    Which levels land on the chart that was drawn.

  README.txt  This file.


A BAND IS SIZED OFF ITS OWN CHART, WHATEVER IS DRAWN

  A weekly band is 0.35 of a normal WEEKLY candle even when it is shown on a
  4-hour chart. So the weekly and the daily candles are fetched every time,
  whichever chart he picked, and only the 4-hour costs a request of its own.

  Sizing a weekly band off 4-hour candles would make it a tenth of its real
  thickness and it would still look like a band.


THE WEEKLY SHOWS EVERYTHING; THE OTHER TWO SHOW ONE LEVEL CLOSELY

  His levels are years apart, so the weekly is the only chart wide enough to
  hold them together. A daily level shows as a thin line on it — correct, and
  a reminder that a daily level is really for looking at on a daily chart.

  150 four-hour candles is about twenty-five days. A weekly level drawn two
  years ago is simply off the top or bottom of that, and the picture draws
  perfectly and comes out looking empty.

  THAT IS WHY Drawn CARRIES A COUNT. The caller says "none of your levels reach
  this far in" rather than sending a chart that looks like the bands failed to
  draw. A picture that looks broken and a picture that is correctly empty are
  the same picture without it.

  The count is an OVERLAP, not a hit on the line. A level drawn above the
  highest candle can still have its lower edge on screen, and that edge is the
  part he is looking for.


WHERE THE FEED'S NAMES FOR THE CHARTS LIVE

  picture.rs has "1week", "1day", "4h" in one small function.

  watch/bands.rs has the same three written out beside the fetch it does. Two
  lists of the same names disagree in the end — pull them onto the one in here
  when that file is next touched.
