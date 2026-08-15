use std::sync::atomic::{AtomicU32, Ordering};

use nsc_core::error::{Answer, Knows};

use super::keep_trying;

/// A trouble that clears in a moment rather than in three seconds.
///
/// The real waits are seconds long, which is right in the field and wrong in a
/// test — a test that sleeps is a test people stop running.
#[derive(Debug)]
enum Pretend {
    Passing,
    Settled,
}

impl Knows for Pretend {
    fn answer(&self) -> Answer {
        match self {
            Pretend::Passing => Answer::TryAgain(std::time::Duration::from_millis(1)),
            Pretend::Settled => Answer::GiveUp,
        }
    }
}

#[tokio::test]
async fn it_stops_the_moment_the_trouble_says_to() {
    static TRIES: AtomicU32 = AtomicU32::new(0);

    let out = keep_trying(5, || async {
        TRIES.fetch_add(1, Ordering::SeqCst);
        Err::<(), _>(Pretend::Settled)
    })
    .await;

    assert!(out.is_err());
    assert_eq!(TRIES.load(Ordering::SeqCst), 1, "asked once, then stopped");
}

// "Keep trying" is not the same as "forever".
#[tokio::test]
async fn it_gives_up_after_the_attempts_it_was_given() {
    static TRIES: AtomicU32 = AtomicU32::new(0);

    let out = keep_trying(3, || async {
        TRIES.fetch_add(1, Ordering::SeqCst);
        Err::<(), _>(Pretend::Passing)
    })
    .await;

    assert!(out.is_err());
    assert_eq!(TRIES.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn a_hiccup_on_the_first_go_does_not_lose_the_job() {
    static TRIES: AtomicU32 = AtomicU32::new(0);

    let out = keep_trying(3, || async {
        if TRIES.fetch_add(1, Ordering::SeqCst) == 0 {
            Err(Pretend::Passing)
        } else {
            Ok(4094)
        }
    })
    .await;

    assert_eq!(out.expect("second go worked"), 4094);
}
