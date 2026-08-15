inbox/ — the bot listening
==========================


WHAT THIS FOLDER IS FOR

  The other side of Telegram. telegram.rs talks; this listens.

  He sends a level from his phone and it lands in config/pairs/. Then it
  draws the pair and sends the picture back, so he can see where the band
  actually sat.


THE FILES

  main.rs           The waiting. Asks Telegram "anything new?", checks it
                    came from him, hands the words to the conversation.

  conversation.rs   Working out what he meant. Which pair, which timeframe,
                    which prices — and what to say back.

  picture.rs        Drawing the pair and sending it.

  talking.rs        Saying things to Telegram, with buttons or without.

  README.txt        This file.


HOW IT GOES

  /level              which pair?        [XAUUSD] [GBPUSD] [+ new pair]
  XAUUSD              which timeframe?   [Weekly] [Daily] [4-hour]
  Weekly              send prices
  4520 4000 3800      saved, and here is the chart          [↩ Undo]

  The pair and the timeframe STICK. Six weekly levels is two taps and six
  numbers.


FOUR THINGS THAT ARE DELIBERATE

  1. THE BUTTONS ARE THE FILES.

     Whatever is in config/pairs/ becomes a button. Not a list in this code —
     that was the mistake settings.rs made, and two lists always disagree in
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
