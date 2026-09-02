recording/ — writing down what rung 3 decided
=============================================


WHAT THIS FOLDER IS FOR

  Every candle rung 3 looks at ends one of two ways: a signal, or a refusal.
  Both get a row.

  CLAUDE.md has asked for the second one since the beginning: "Rejected setups
  get saved, not thrown away. Save which layer rejected them. Those rows answer
  'why did nothing fire this week?' and they are the 'don't take this' examples
  the Phase 4 model needs."

  A quiet week and a broken bot look identical from outside. So do "nothing
  printed" and "forty shapes printed and not one was near a level" -- and those
  are completely different problems.


THE FILES

  mod.rs        The front door.

  version.rs    A short hash of the settings that produced the decision.
                Without it, "these came back at 38%" is unanswerable: 38%
                under WHICH thresholds? Worked out once at startup, because
                the bot refuses to reload settings while it runs.

  features.rs   Everything the bot saw, as it saw it. Written once and never
                worked out again -- recalculated later against updated
                chart-reading code it would train a model on inputs the live
                bot never produced, and NOTHING DETECTS THAT. Both sides keep
                working and only the scores are wrong.

  writing.rs    Turning a decision into a row. Nothing here can end the run:
                a row that will not write is a gap in the history, not a
                reason to stop watching his levels.

  asking.rs     The two buttons under a setup -- took it, skipped it.

                THEY ARE THEIR OWN MESSAGE, and they have to be: a setup goes
                out as three pictures in one sendMediaGroup, and Telegram does
                not allow buttons on a media group. So the pictures land and a
                one-line message follows carrying the buttons.

                That line NAMES the setup. Two setups on one pair in an hour
                would otherwise be two identical questions, and the label
                would land on whichever he happened to tap.

  README.txt    This file.


WHAT IS NOT WRITTEN DOWN

  A candle with no shape on it at all.

  That is nearly every candle. A row for each would make `rejections` far
  larger than `candles` while saying less -- and "there was no shape on it"
  can be worked out from the candle any time.

  What cannot be worked out afterwards is the rest: a shape the rules refused,
  or one that printed nowhere near a level. Those depend on the settings that
  were live at that moment, and settings change.

  `Refused::worth_keeping` in nsc-strategy is where that line is drawn, and a
  test pins it.


THE TWO SIDES CARRY THE SAME FEATURES

  Exactly the same keys, signal and refusal alike. They are the two halves of
  one dataset -- what to take and what not to -- and a different shape on each
  side makes them unusable together, which is the only use either has.
