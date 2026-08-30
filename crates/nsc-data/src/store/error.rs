//! What can go wrong, and the one question each answers: **try again, or give
//! up?**

use thiserror::Error;

/// Trouble with the record.
///
/// **Two kinds, and telling them apart is the whole point.** A dropped
/// connection clears on its own and is worth retrying; a table that is not
/// there will still not be there in ten seconds. Lumping them together makes
/// the bot retry a migration it never ran, forever, and it looks exactly like
/// a slow database.
#[derive(Debug, Error)]
pub enum StoreError {
    /// **The database is not reachable.** Worth another go.
    #[error("could not reach the database: {detail}")]
    Unreachable { detail: String },

    /// **The query itself is wrong**, or the schema is not what it expects.
    /// Trying again changes nothing.
    #[error("the database refused that: {detail}")]
    Refused { detail: String },

    /// **The migrations did not run.** Its own case because the fix is one
    /// command and the message should say so.
    #[error("the tables are not there — run `sqlx migrate run`: {detail}")]
    NotSetUp { detail: String },
}

impl StoreError {
    /// **Is it worth trying again?**
    ///
    /// The one question a caller actually has. `Unreachable` yes, the other
    /// two no — and a bot that retries a bad query forever looks identical to
    /// one waiting on a slow network.
    pub fn worth_retrying(&self) -> bool {
        matches!(self, StoreError::Unreachable { .. })
    }
}

impl From<sqlx::Error> for StoreError {
    /// **Sorted here, once.** Every caller would otherwise have to know which
    /// `sqlx::Error` means what, and they would not agree.
    fn from(trouble: sqlx::Error) -> Self {
        let detail = trouble.to_string();

        match &trouble {
            sqlx::Error::Io(_) | sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => {
                StoreError::Unreachable { detail }
            }

            // **42P01 is "undefined table".** It is the one wrong answer with
            // an obvious fix, so it gets its own message rather than being
            // filed under "the database refused that".
            sqlx::Error::Database(problem) if problem.code().as_deref() == Some("42P01") => {
                StoreError::NotSetUp { detail }
            }

            _ => StoreError::Refused { detail },
        }
    }
}

impl From<sqlx::migrate::MigrateError> for StoreError {
    fn from(trouble: sqlx::migrate::MigrateError) -> Self {
        StoreError::NotSetUp {
            detail: trouble.to_string(),
        }
    }
}
