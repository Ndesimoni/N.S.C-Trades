use std::sync::atomic::{AtomicU32, Ordering};

use super::{Answer, Knows, keep_trying};
use crate::error::FeedError;
use crate::error::SendError;

// ── The distinction the whole thing exists for ──

// A wrong key is wrong on the fifth go. Retrying it forever looks exactly like
// a dead connection, and that is the failure this rule was written for.
#[test]
fn a_bad_key_is_never_worth_trying_again() {
    let refused = FeedError::Refused {
        code: 401,
        message: "apikey is incorrect".into(),
    };

    assert_eq!(refused.answer(), Answer::GiveUp);
    assert_eq!(FeedError::NoKey.answer(), Answer::GiveUp);
}

// A dropped line clears on its own.
#[test]
fn a_dropped_line_is_worth_trying_again() {
    let trouble = FeedError::Unreachable("connection reset".into());

    assert!(trouble.answer().worth_trying_again());
}

// They have TOLD us to slow down, so slow down properly rather than hammering
// — a longer wait than a hiccup gets.
#[test]
fn being_told_to_slow_down_waits_longer_than_a_hiccup() {
    let busy = FeedError::Refused {
        code: 429,
        message: "too many requests".into(),
    };
    let hiccup = FeedError::Unreachable("timed out".into());

    assert!(busy.answer().wait() > hiccup.answer().wait());
}

// Their end falling over is not our problem to solve, only to wait out.
#[test]
fn their_end_falling_over_is_worth_trying_again() {
    for code in [500, 502, 503] {
        let trouble = FeedError::Refused {
            code,
            message: "server error".into(),
        };
        assert!(trouble.answer().worth_trying_again(), "{code}");
    }
}

// A pair that is not on the plan will never be on the plan by asking again.
#[test]
fn a_pair_they_do_not_carry_is_settled() {
    let trouble = FeedError::Refused {
        code: 404,
        message: "symbol not found".into(),
    };

    assert_eq!(trouble.answer(), Answer::GiveUp);
}

// ── Telegram says it in words, not codes ──

#[test]
fn telegram_being_busy_is_worth_trying_again() {
    let busy = SendError::Refused("Too Many Requests: retry after 30".into());
    let bad_token = SendError::Refused("Unauthorized".into());

    assert!(busy.answer().worth_trying_again());
    assert_eq!(bad_token.answer(), Answer::GiveUp);
}

#[test]
fn a_missing_picture_is_settled() {
    let missing = SendError::NoPicture {
        path: "preview/chart.png".into(),
        detail: "no such file".into(),
    };

    assert_eq!(missing.answer(), Answer::GiveUp);
}

// ── The retrying itself ──

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
