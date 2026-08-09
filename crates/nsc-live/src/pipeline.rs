//! The signal pipeline. One closed candle in, at most one signal out.
//!
//! ```text
//!   candle closed
//!     → build the bigger timeframes
//!     → read the chart              (nsc-ta)
//!     → apply your rules            (nsc-strategy)   ── the clean part
//!     → no setup? record why, stop
//!     → risk and exposure checks    (nsc-risk)
//!     → news blackout               (nsc-news)
//!     → model score                 (nsc-ai)     [Phase 4]
//!     → AI review                   (nsc-ai)     [Phase 5]
//!     → draw the chart              (nsc-chart)
//!     → send it                     (nsc-telegram)
//!     → save the signal and everything the bot saw
//! ```
//!
//! Notice where the clean part ends. Everything up to and including applying
//! your rules is the identical code the backtester runs. Everything after is
//! sending and saving. Keeping that line clean is what makes backtest results
//! mean anything.
//!
//! Every stage records its decision. A setup killed by the news filter is
//! saved with that reason, not dropped — those rejections are half your
//! dataset.
