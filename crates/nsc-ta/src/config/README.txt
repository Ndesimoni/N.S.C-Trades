config/ — the settings from ta.toml, as types
=============================================


WHAT THIS FOLDER IS FOR

  Describing what the settings in config/ta.toml look like, and checking they
  make sense.

  Note this is NOT the config/ folder at the root of the project. That one
  holds the actual TOML files a trader edits. This one is the code that gives
  those settings a shape.


THE FILES

  mod.rs           The front door.

  settings.rs      TaSettings. Holds all the sections together, and the one
                   validate() call that checks the lot.

  swings.rs        SwingSettings, from the [swings] section.

  levels.rs        LevelSettings, from the [levels] section.

  structure.rs     StructureSettings, from the [structure] section.

  indicators.rs    IndicatorSettings, from the [indicators] section.

  tests.rs         Eight tests.

  README.txt       This file.


WHAT IS NOT IN HERE

  strong_touches — how many touches makes a level worth trading.

  It used to be in ta.toml [levels] and it moved to strategy.toml, because it
  is a trading opinion rather than part of drawing a level. This crate counts
  the touches; the rules decide what a lot of touches is worth.


HOW THEY FIT TOGETHER

      swings.rs     ─┐
      levels.rs      │
      structure.rs   ├─►  settings.rs      TaSettings holds all four
      indicators.rs ─┘

      all of them   ─►  error.rs           a bad setting is a BadSetting

      mod.rs        ─►  lets the outside world see all three


WHY THIS CRATE DOES NOT READ THE FILE ITSELF

  nsc-ta is not allowed to touch the outside world. No files, no internet, no
  clock.

  So it does not open ta.toml. It describes the shape, and whoever starts the
  program reads the TOML, fills these in, and hands them over.

  That rule exists so the backtester and the live bot run the same analysis
  code. It also happens to make testing easy: hand the analysis made-up
  settings and check the answers, with no config file involved.


CHECKED ONCE, AT STARTUP

  Call TaSettings::validate() after loading. Once.

  Not on every candle. Checking a setting four hundred thousand times to get
  the same answer is a waste — and finding out at candle 400,000 that the
  lookback is zero is far too late anyway.

  validate() stops at the first problem instead of collecting them all. One
  wrong setting makes the results meaningless, so there is nothing to gain by
  carrying on.


A TYPO MUST STOP THE PROGRAM

  Every settings struct has deny_unknown_fields on it.

  Without that, writing "lookbak = 3" in ta.toml would be silently ignored.
  The setting stays at whatever it was. You think you changed it. You run a
  backtest, see identical results, and spend an evening wondering why the
  lookback does not seem to do anything.

  With it, the program refuses to start and names the field.

  There is a test for this called a_misspelled_setting_is_refused. It is not
  testing our code — it is making sure deny_unknown_fields is still there.


NO DEFAULTS. EVER.

  Every setting must be present in the TOML file.

  It is tempting to give lookback a default of 3 so the config file can be
  shorter. Do not.

  Then there are two answers to "what is the lookback": the one in ta.toml
  and the one in the code. When they disagree, the code wins silently, while
  you are sitting there reading the file.


ONLY WHAT HAS CODE BEHIND IT

  ta.toml has seven sections. This folder covers two.

  The rest get added as the modules that use them are built. Settings written
  for code that does not exist go stale before anyone reads them, and then
  you cannot tell which settings are real.
