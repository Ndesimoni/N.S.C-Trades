//! Working out what he meant, and answering.
//!
//! Everything he sends lands here: a command, a button he tapped, or a line of
//! prices. There is no session and no state on Telegram's side, so **where he
//! is in a flow is remembered here** and nowhere else.
//!
//!   adding.rs    where he is — the pair, the chart, what he is part-way into
//!   route.rs     what a message means, given where he is
//!   saving.rs    a line of prices, saved, and said back to him
//!   reading.rs   pulling the numbers out of a message

mod adding;
mod naming;
mod reading;
mod route;
mod saving;

pub use adding::Adding;
pub use route::handle;
