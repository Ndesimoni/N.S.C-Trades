//! The numbers, all measured in R.
//!
//! R means multiples of what you risked. Using R instead of pounds or dollars
//! makes results comparable across pairs and across account sizes.
//!
//! Works out: how many trades, win rate, average result, profit factor, worst
//! drawdown, longest losing run, average time in a trade, and how far price
//! ran in your favour and against you before resolving.
//!
//! **Average result beats win rate.** Winning 35% of the time at 3R is a much
//! better system than winning 70% at 0.4R — and win rate is the number that
//! will tempt you to ruin the first one.
//!
//! The favourable/adverse numbers are the practical diagnostic. They tell you
//! whether your stops are getting clipped just before the move, or your
//! targets are sitting past where price actually turns.
//!
//! Any number worked out from fewer than about 100 trades should be shown with
//! the trade count next to it. It is a story with a decimal point.
