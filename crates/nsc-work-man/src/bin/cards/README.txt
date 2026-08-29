cards/ — draw any card, without waiting for the market
======================================================


WHAT THIS IS FOR

      cargo run -p nsc-work-man --bin cards -- XAUUSD              approaching
      cargo run -p nsc-work-man --bin cards -- XAUUSD 4120         in the zone
      cargo run -p nsc-work-man --bin cards -- XAUUSD 4120 found   already in
      cargo run -p nsc-work-man --bin cards -- XAUUSD close        a close
      cargo run -p nsc-work-man --bin cards -- setup
      cargo run -p nsc-work-man --bin cards -- news
      cargo run -p nsc-work-man --bin cards -- setup
      cargo run -p nsc-work-man --bin cards -- news busy
      cargo run -p nsc-work-man --bin cards -- heartbeat
      cargo run -p nsc-work-man --bin cards -- armed
      cargo run -p nsc-work-man --bin cards -- trouble down|back|stopped

  EVERY MESSAGE THE BOT CAN SEND, on demand.

  Changing how a card looks means looking at it, and the market reaches a
  level when it feels like it. Some of these would otherwise take a week to
  see — a quiet-day heartbeat, or the line going down and coming back.


THE FILES

  main.rs     Which card he asked for, and the candles the zone ones need.

  found.rs    THE SETUP CARD, rung 3. No TWS either -- it runs on the two
              gold candles off his own screenshot, 19 and 20 August 2026.

              THE CANDLES ARE REAL AND THE ZONE IS PLACED FOR THE PICTURE.
              Said out loud in the file rather than left to be assumed: a real
              zone has to be where price actually was.

  soon.rs     THE NEWS CARD, drawn against the real calendar. The only card
              here that needs no TWS at all -- the economic calendar is a
              plain web page with no key on it.

              `news` draws the next release he would be told about. `news
              busy` draws the BUSIEST group of the week, which is the one
              worth looking at: one release is the easy case, and four at once
              is what the layout has to survive.

  zone.rs     The two zone cards — price at a level, and a candle at one.

  beat.rs     The heartbeat, the armed card, and the three trouble cards.

  README.txt  This file.


IT USES THE REAL WORDING

  The captions come from the same functions the bot calls — levels::caption,
  card::caption, when::beat_words.

  The trouble caption was written twice for a while, once here and once in the
  bot, and this copy won. So all three states arrived captioned identically
  and none of them said what had happened. A preview that writes its own words
  is showing him something that will never be sent.


AND IT DRAWS THE REAL STATE

  With no price it puts one just outside the pair's first band — the state
  hardest to draw, where price is close enough to the edge that the labels
  crowd into each other.

  The recovery card passes no detail, because the real one has nothing left to
  explain. Given a made-up one it would look better than it ever will.
