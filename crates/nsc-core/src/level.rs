//! Support and resistance as zones, not exact prices.
//!
//! A level is a price band built from nearby swing points, carrying:
//!   - `touches`    how many times price respected it
//!   - `last_test`  when it was last tested
//!   - `strength`   a score from touches, age, and which timeframe it came from
//!   - `exhausted`  tested so often that it is now more likely to break
//!
//! Zones instead of exact prices, for two reasons. Price does not turn at an
//! exact number. And an exact price forces you to add a fixed pip tolerance
//! every time you compare against it — which, as everywhere else, stops
//! working when you add a second pair.
