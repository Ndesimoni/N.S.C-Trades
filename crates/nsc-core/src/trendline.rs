//! A line drawn through two or more swing points, and whether it still holds.
//!
//! Two points make a line. The third touch is what makes it worth trading.
//!
//! A trendline is broken by candles **closing** past it, not by wicks poking
//! through. How much tolerance is in `config/ta.toml`.
//!
//! Lines are stored as price-and-time, not as a slope on screen, so the line
//! means the same thing no matter how a chart happens to be drawn.
