use super::{Answer, Knows};

/// A trouble that stands in for a real one, so this file can test the
/// QUESTION rather than any particular answer to it.
#[derive(Debug)]
enum Pretend {
    Passing,
    Settled,
}

impl Knows for Pretend {
    fn answer(&self) -> Answer {
        match self {
            Pretend::Passing => Answer::soon(),
            Pretend::Settled => Answer::GiveUp,
        }
    }
}

// ── The question itself ──

#[test]
fn a_passing_trouble_is_worth_trying_again() {
    assert!(Pretend::Passing.answer().worth_trying_again());
    assert!(Pretend::Passing.answer().wait().is_some());
}

#[test]
fn a_settled_one_is_not_and_has_nothing_to_wait_for() {
    assert_eq!(Pretend::Settled.answer(), Answer::GiveUp);
    assert!(Pretend::Settled.answer().wait().is_none());
}

// Being told to slow down waits longer than a hiccup does. They have SAID to
// slow down, so hammering is both rude and pointless.
#[test]
fn being_told_to_slow_down_waits_longer_than_a_hiccup() {
    assert!(Answer::in_a_while().wait() > Answer::soon().wait());
}
