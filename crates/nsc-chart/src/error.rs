//! Things that go wrong drawing the chart.
//!
//! A drawing failure must never swallow the signal. If the image cannot be
//! made, the text message still goes out with a note. A signal you can read
//! beats no signal because a font was missing.
//!
//! The realistic causes are dull: too few candles to draw a sensible window,
//! or a font that exists on your laptop and not on the server.
