//! Which failures are worth another go.

use nsc_core::error::{Answer, Knows};

use super::FeedError;

// A wrong key is wrong on the fifth go. Retrying it forever looks exactly like
// a dead connection, and that is the failure the whole rule was written for.
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
