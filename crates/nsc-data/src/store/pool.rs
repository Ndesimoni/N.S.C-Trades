//! Opening the record, and putting the tables there.

use std::time::Duration;

use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::{Pool, migrate::Migrator};

use super::StoreError;

/// The connection pool, handed round rather than opened per query.
pub type Store = Pool<Postgres>;

/// **The migrations are baked into the binary.** Shipping a folder of SQL that
/// has to travel with the executable is one more thing to get wrong on a
/// server; this way the binary always carries the schema it expects.
/// **The path is relative to THIS crate, and `sqlx migrate run` is relative to
/// the workspace root.** They point at the same folder by two different routes
/// — the one folder `CLAUDE.md` already names, `migrations/` at the top.
static MIGRATIONS: Migrator = sqlx::migrate!("../../migrations");

/// How long to wait for a connection before saying the database is unreachable.
///
/// **Short on purpose.** The bot has a price line to answer; a query that has
/// not come back in five seconds is not going to save the candle it was for,
/// and the caller needs to hear "unreachable" while that is still useful.
const PATIENCE: Duration = Duration::from_secs(5);

/// How many connections to hold.
///
/// **Five, and the reason is what runs at once.** The price loop, the inbox,
/// the candle watcher, the news watcher and one spare. More would be idle
/// sockets; fewer and two of them wait on each other.
const HOLD: u32 = 5;

/// Opens the record and brings the schema up to date.
///
/// **Migrations run on the way in, every start.** They are cheap when there is
/// nothing to do, and the alternative is a bot that starts happily against a
/// schema older than the code — which fails later, somewhere else, with a
/// message about a column.
///
/// `url` is a Postgres connection string. **It never gets logged**: it has the
/// password in it, and this project has already been bitten once by a secret
/// travelling in an error message.
pub async fn open(url: &str) -> Result<Store, StoreError> {
    let pool = PgPoolOptions::new()
        .max_connections(HOLD)
        .acquire_timeout(PATIENCE)
        .connect(url)
        .await
        .map_err(|trouble| StoreError::Unreachable {
            // **Not the url.** `sqlx` puts it in its own message and that is
            // where the password would end up.
            detail: strip(&trouble.to_string(), url),
        })?;

    MIGRATIONS.run(&pool).await?;

    Ok(pool)
}

/// Takes the connection string back out of a message.
///
/// **The rule "never print the url, the key is in it" was written down here in
/// August, followed on the happy path, and never applied to the error path —
/// which is the one that prints.** So it is done in code rather than
/// remembered.
fn strip(message: &str, url: &str) -> String {
    message.replace(url, "<the database url>")
}
