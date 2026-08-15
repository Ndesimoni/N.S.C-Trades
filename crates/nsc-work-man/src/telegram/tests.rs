//! Which failures are worth another go.
//!
//! **Telegram says it in words, not codes.** There is no 429 to match on — it
//! answers 200 with `ok: false` and a description, so the words are what there
//! is to go by.

use nsc_core::error::{Answer, Knows};

use super::SendError;

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

#[test]
fn a_missing_token_is_settled() {
    assert_eq!(
        SendError::NotSet("TELEGRAM_BOT_TOKEN").answer(),
        Answer::GiveUp
    );
}
