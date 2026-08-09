//! The shared things every background job needs.
//!
//! Database pool, Redis connection, the loaded settings, the list of pairs,
//! and the feature switches. Built once at startup and shared.
//!
//! Settings are deliberately **not** reloaded while running. Changing rules
//! under a running bot means signals within one session came from different
//! rules, and no amount of analysis afterwards can untangle which. Restart to
//! change behaviour — the restart is how you know.
