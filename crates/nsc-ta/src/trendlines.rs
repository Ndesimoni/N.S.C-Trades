//! Drawing trendlines and tracking whether they still hold.
//!
//! Lines are fitted through confirmed swing points, needing a minimum number
//! of touches. Two points make a line; the third is what makes it worth
//! trading.
//!
//! A line is broken by candles **closing** past it, not by wicks poking
//! through. Wicks through trendlines are common and usually mean nothing;
//! treating them as breaks throws away useful lines constantly.
//!
//! Lines are fitted only to swing points, never as a best-fit through every
//! candle. A best-fit line is a statistics exercise that no trader would ever
//! draw, and entering trades off it produces signals you will not recognise as
//! yours.
