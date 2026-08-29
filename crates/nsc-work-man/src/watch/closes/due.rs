//! When a pair's next candle is worth asking about.
//!
//! **Nothing tells us a candle closed.** The price line sends prices, about
//! one a second, and never says "the 14:00 hourly just finished". Approaching
//! a zone, reaching it and sitting in it all cost nothing — they come off that
//! line. Only *what the candle did* needs a request.
//!
//! So it used to ask every ten minutes. A 4-hour candle closes six times a
//! day, so about **140 of every 144 asks found nothing new**.
//!
//! ## It still does not work out where candles close
//!
//! That rule stands, and it is the important one: nobody has measured where
//! this feed puts its 4-hour boundaries, and guessing wrong reports a candle
//! that has not finished — the mistake that makes results look better rather
//! than broken.
//!
//! This does not guess. The feed hands back a candle **stamped 14:00 on the
//! 4-hour**, which is it telling us where its own boundary is. The next is due
//! at 18:00. That is reading, not calculating, and the returned stamp is still
//! what decides whether a candle has finished.

use chrono::{DateTime, Duration, Utc};
use nsc_core::candle::Bar;

/// The soonest two asks about the same pair and interval may be.
///
/// **A feed handing back a stale stamp must not become a tight loop.** Every
/// moment worked out below could be in the past if the newest candle we were
/// given is older than it should be, and then "ask when the next is due" means
/// "ask again immediately, forever".
const FLOOR: Duration = Duration::minutes(1);

/// When this pair and interval is next worth a request.
///
/// **One moment matters now: the point the candle finishes.**
///
/// There used to be two — the close, and a third of the way in, when the
/// mid-candle "so far" card was due. That card went on 27 August 2026, and
/// with it the only reason to wake before a candle had ended.
pub(super) fn worth_asking_again(bars: &[Bar], minutes: i64, now: DateTime<Utc>) -> DateTime<Utc> {
    let floor = now + FLOOR;

    // Newest first, so this is the candle currently running — or the one that
    // has just finished, if no price has landed in the new one yet.
    let Some(opened) = bars.first().and_then(|bar| bar.opened_at().ok()) else {
        return floor;
    };

    let Some(step) = Duration::try_minutes(minutes) else {
        return floor;
    };

    // This candle's close, then the next one's — because the newest candle we
    // were handed may already be finished.
    let moments = [opened + step, opened + step + step];

    moments
        .into_iter()
        .find(|at| *at > floor)
        .unwrap_or(floor)
        .max(floor)
}

/// When to ask again, given whether everything that had to be said was said.
///
/// **A close that would not send is the one thing worth hurrying back for.**
/// Waiting for the next candle would be right if the request had done its job
/// — but it did not, and on the 4-hour that is four hours before he hears
/// about a candle the bot has already read.
///
/// Nothing is marked as told when a card fails, so coming back early is enough
/// to put it right.
pub(super) fn when_next(
    all_sent: bool,
    when_the_candle_is_due: DateTime<Utc>,
    now: DateTime<Utc>,
) -> DateTime<Utc> {
    if all_sent {
        return when_the_candle_is_due;
    }

    now + FLOOR
}

#[cfg(test)]
mod tests {
    use super::{when_next, worth_asking_again};
    use chrono::{DateTime, Duration, Utc};
    use nsc_core::candle::Bar;
    use rust_decimal::Decimal;

    fn at(text: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(text)
            .expect("a real moment")
            .with_timezone(&Utc)
    }

    /// One candle, opened when it says. Only the stamp matters here.
    fn opened(stamp: &str) -> Bar {
        Bar {
            datetime: stamp.to_string(),
            open: Decimal::ONE,
            high: Decimal::ONE,
            low: Decimal::ONE,
            close: Decimal::ONE,
        }
    }

    /// **The 4-hour, which is where the waste was.** A candle opened at 12:00
    /// runs to 16:00. Asked at 12:30, the next thing worth asking about is the
    /// close at 16:00 — not ten minutes from now, and no longer a mid-candle
    /// look at 13:20 either.
    #[test]
    fn it_waits_for_the_close_and_nothing_sooner() {
        let due = worth_asking_again(
            &[opened("2026-08-17 12:00:00")],
            240,
            at("2026-08-17T12:30:00Z"),
        );

        assert_eq!(due, at("2026-08-17T16:00:00Z"));
    }

    /// **Three and a half hours of silence, and that is correct.** The
    /// mid-candle card was the only thing that ever broke it, and it went on
    /// 27 August 2026.
    #[test]
    fn it_does_not_wake_part_way_through_any_more() {
        let due = worth_asking_again(
            &[opened("2026-08-17 12:00:00")],
            240,
            at("2026-08-17T13:30:00Z"),
        );

        assert_eq!(due, at("2026-08-17T16:00:00Z"));
    }

    /// The newest candle handed back can already be finished — no price has
    /// landed in the new one yet. Then it is the NEXT candle's close that
    /// matters.
    #[test]
    fn a_finished_candle_points_at_the_next_one() {
        let due = worth_asking_again(
            &[opened("2026-08-17 12:00:00")],
            60,
            at("2026-08-17T13:05:00Z"),
        );

        // The 12:00 candle has closed, so the next moment is the 13:00 one
        // closing at 14:00. It used to be 13:20 — that was its mid-candle
        // look, and that card went on 27 August 2026.
        assert_eq!(due, at("2026-08-17T14:00:00Z"));
    }

    /// **A stale stamp must not become a tight loop.** Every moment worked out
    /// from a candle old enough is in the past.
    #[test]
    fn a_stamp_from_last_week_still_waits() {
        let now = at("2026-08-17T12:00:00Z");
        let due = worth_asking_again(&[opened("2026-08-10 12:00:00")], 60, now);

        assert!(due >= now + Duration::minutes(1), "got {due}");
    }

    /// No candles at all — the feed answered with nothing. Wait the floor
    /// rather than hammering it.
    #[test]
    fn nothing_back_waits_the_floor() {
        let now = at("2026-08-17T12:00:00Z");

        assert_eq!(worth_asking_again(&[], 60, now), now + Duration::minutes(1));
    }

    /// Everything sent, so wait for the candle the feed pointed at.
    #[test]
    fn all_said_waits_for_the_next_candle() {
        let now = at("2026-08-17T12:00:00Z");
        let candle = at("2026-08-17T16:00:00Z");

        assert_eq!(when_next(true, candle, now), candle);
    }

    /// **A card that would not send comes back in a minute, not in four
    /// hours.** Nothing is marked as told when a send fails, so an early
    /// return is all it takes — but on the 4-hour, waiting for the next candle
    /// means he hears about this one four hours after the bot read it.
    #[test]
    fn something_unsaid_comes_back_soon() {
        let now = at("2026-08-17T12:00:00Z");
        let candle = at("2026-08-17T16:00:00Z");

        assert_eq!(when_next(false, candle, now), now + Duration::minutes(1));
    }

    /// A stamp the feed sends in a shape we cannot read is not a reason to ask
    /// again instantly.
    #[test]
    fn a_stamp_we_cannot_read_waits_the_floor() {
        let now = at("2026-08-17T12:00:00Z");
        let due = worth_asking_again(&[opened("last Tuesday")], 60, now);

        assert_eq!(due, now + Duration::minutes(1));
    }
}
