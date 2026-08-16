TESTS FOR THE CARDS
===================

Fourteen tests, and every one of them exists because something actually went
wrong. None of them draw a card -- that needs Chrome, and Chrome is not on a
build machine.


THE FILES

  drawing.rs   The height read out of a template, and the page Chrome is
               handed.

               The bug: style.css is inlined at the top of every card, so
               reading the FIRST --card-height found meant a card could never
               override the shared one. Last wins now, which is what a browser
               does.

  words.rs     Rounding on the way out, and the line that goes under the
               picture.

               The bug: the caption was written twice, once for the real
               message and once for the preview, and the preview used its own.
               All three states arrived captioned identically.

  safety.rs    What must never travel on a card, and what must never be stale.

               Two bugs. The bot token appeared in full on a trouble card,
               because reqwest puts the URL in its error text. And a card was
               only checked for EXISTING after Chrome ran -- but the last one
               was still sitting there, so a failed draw sent yesterday's
               picture under today's caption.
