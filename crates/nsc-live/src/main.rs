//! # nsc-live — the bot
//!
//! Loads settings, connects to Postgres and Redis, starts the background
//! jobs, and waits for shutdown.
//!
//! ## The jobs
//!
//! ```text
//!   feed      ──→ broker connection, builds candles, announces closed ones
//!   pipeline  ──→ read chart → apply rules → check → send
//!   tracker   ──→ follows open signals until they hit stop or target
//!   news      ──→ keeps the calendar up to date
//!   health    ──→ is the feed actually alive?
//! ```
//!
//! ## What actually goes wrong in production
//!
//! Not a crash. A crash is obvious and gets restarted.
//!
//! The real failure is a feed that quietly stops while the process keeps
//! running and looking fine. A bot with nothing to say looks exactly like a
//! quiet market. You can lose a week before you notice.
//!
//! That is why the health job exists, and why every job has to report that it
//! is *receiving candles* — not merely that it is still running. Those are
//! different claims.

mod pipeline;
mod shutdown;
mod state;
mod tasks;

fn main() {
    // Phase 0: load settings, start logging, connect, spawn the jobs.
    todo!("see tasks/ for how this is meant to be wired up")
}
