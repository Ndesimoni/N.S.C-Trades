//! Waiting for Chrome, but not forever.

use std::time::{Duration, Instant};

use super::CardError;

/// How long Chrome gets before it is stopped.
///
/// **A card takes about two seconds.** A minute is far past generous, and it
/// is not "forever" — which is what it had, and what wedged the whole bot: a
/// Chrome that draws the picture and then never exits leaves the call waiting
/// on it waiting for good. Nothing answered, not even `/help`, and nothing
/// said why.
const PATIENCE: Duration = Duration::from_secs(60);

/// How often to look and see whether Chrome has finished.
const GLANCE: Duration = Duration::from_millis(100);

/// Waits for Chrome, but not forever.
///
/// **`output()` waits for good**, and Chrome can finish its work and then not
/// exit. When that happened the bot stopped answering anything at all — the
/// thread was stuck in here, and nothing in any log said so.
///
/// Killed rather than left, because a Chrome that will not exit is one more
/// every time a card is drawn.
pub(super) fn wait_for(chrome: std::process::Child) -> Result<String, CardError> {
    wait_up_to(chrome, PATIENCE)
}

/// The waiting itself, with the deadline handed in so a test can use a short
/// one. Sixty seconds is the right answer for a card and the wrong one for a
/// test that has to prove the deadline works.
fn wait_up_to(mut chrome: std::process::Child, patience: Duration) -> Result<String, CardError> {
    let give_up_at = Instant::now() + patience;

    // **Read on its own thread, while Chrome is still running.**
    //
    // A pipe holds about 64k. Left unread until the process exits, a Chrome
    // that said more than that would block trying to say it — and then it
    // never exits, and we are back to waiting on something that cannot
    // finish. It says about 2k on a normal run, so this has never bitten;
    // it is the same shape as the bug that wedged the bot, which is reason
    // enough not to leave it lying there.
    let talking = chrome.stderr.take().map(|mut errors| {
        std::thread::spawn(move || {
            use std::io::Read;

            let mut said = String::new();
            let _ = errors.read_to_string(&mut said);
            said
        })
    });

    let heard = || {
        talking
            .map(|thread| thread.join().unwrap_or_default())
            .unwrap_or_default()
    };

    loop {
        match chrome.try_wait() {
            Err(trouble) => return Err(CardError::DrewNothing(trouble.to_string())),

            // Finished on its own. Whatever it said is the reason if no
            // picture appeared.
            Ok(Some(_)) => return Ok(heard()),

            Ok(None) if Instant::now() >= give_up_at => {
                let _ = chrome.kill();
                let _ = chrome.wait();

                return Err(CardError::DrewNothing(format!(
                    "Chrome had not finished after {} seconds and was stopped.\n{}",
                    patience.as_secs(),
                    heard(),
                )));
            }

            Ok(None) => std::thread::sleep(GLANCE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CardError, wait_up_to};
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    fn started(program: &str, args: &[&str]) -> std::process::Child {
        Command::new(program)
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("it should start")
    }

    /// **The guard that the whole bot rests on.**
    ///
    /// A Chrome that draws the picture and then never exits left the thread
    /// waiting here for good — and because that call blocks, the bot answered
    /// nothing at all. No error, no log line, just silence.
    #[test]
    fn something_that_will_not_finish_is_stopped() {
        let began = Instant::now();
        let answer = wait_up_to(started("sleep", &["30"]), Duration::from_millis(200));

        assert!(began.elapsed() < Duration::from_secs(5), "it must not wait");

        match answer {
            Err(CardError::DrewNothing(why)) => {
                assert!(why.contains("stopped"), "it should say so, got: {why}")
            }
            other => panic!("it should have given up, got {other:?}"),
        }
    }

    /// **And it is really killed, not just given up on.** One Chrome left
    /// running per card is how a machine ends up with twenty-five of them,
    /// which is what happened.
    #[test]
    fn the_one_it_stopped_is_really_gone() {
        let child = started("sleep", &["30"]);
        let id = child.id();

        let began = Instant::now();
        let _ = wait_up_to(child, Duration::from_millis(200));

        // **Killed, not waited out.** Without this, dropping the kill and
        // leaving the wait still ends with the process gone — thirty seconds
        // later, having held the thread the whole time.
        assert!(began.elapsed() < Duration::from_secs(5), "it waited it out");

        // `kill -0` asks whether a process is there without touching it. It
        // was waited on as well as killed, so the id is fully gone.
        let still_there = Command::new("kill")
            .args(["-0", &id.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("kill should run")
            .success();

        assert!(!still_there, "process {id} was left running");
    }

    /// Something that finishes on its own is not touched, and what it said
    /// comes back — that is the only clue when no picture appears.
    #[test]
    fn something_that_finishes_hands_back_what_it_said() {
        let said = wait_up_to(
            started("sh", &["-c", "echo trouble here >&2"]),
            Duration::from_secs(10),
        )
        .expect("it finished on its own");

        assert!(said.contains("trouble here"), "got: {said}");
    }
}
