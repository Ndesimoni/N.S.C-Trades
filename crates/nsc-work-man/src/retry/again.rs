//! The trying itself.

use nsc_core::error::Knows;

/// Does the job, and tries again if the trouble says it is worth it.
///
/// Gives up the moment the trouble says to — a wrong key is wrong on the
/// fifth go — and gives up after `attempts` even when it says otherwise,
/// because "keep trying" is not the same as "forever".
///
/// The wait doubles each time. Their end being busy is rarely fixed by asking
/// again immediately.
pub async fn keep_trying<T, E, F, Fut>(attempts: u32, mut job: F) -> Result<T, E>
where
    E: Knows,
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut left = attempts.max(1);

    loop {
        let trouble = match job().await {
            Ok(done) => return Ok(done),
            Err(trouble) => trouble,
        };

        left -= 1;

        let Some(wait) = trouble.answer().wait() else {
            return Err(trouble);
        };

        if left == 0 {
            return Err(trouble);
        }

        tokio::time::sleep(wait * (attempts - left)).await;
    }
}
