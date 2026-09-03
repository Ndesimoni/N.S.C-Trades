//! Turning a decision into a row, and never letting that end the run.

use chrono::{DateTime, Utc};
use nsc_core::candle::Bar;
use nsc_core::levels::Pair;
use nsc_data::source::Interval;
use nsc_data::store::{self, Seen, Store, Turned};
use nsc_strategy::{Refused, Signal};
use rust_decimal::Decimal;
use std::path::Path;

use super::asking;
use super::features::{FEATURES_VERSION, of_the_candle, with_the_band};

/// **When the bot could first KNOW about this candle** — the moment it closed.
///
/// ## Not `Utc::now()`, and this was written the wrong way first
///
/// The look runs every ten minutes, so `now` is the moment the POLL happened:
/// up to ten minutes after the candle closed, and a different gap every time.
///
/// The design says `at` is *"the honest one and it is the one an outcome must
/// be measured from"*. Measured from a poll time, an outcome starts late by a
/// random amount — and a BACKTEST would compute the close, so the two would
/// disagree while both looked fine. That is the mismatch `CLAUDE.md` refuses:
/// *"never write 'if we're backtesting, do this instead'"*, and a value that
/// can only exist live is the same fault wearing different clothes.
///
/// The close is `opened_at` plus the timeframe. Deterministic, and the same
/// number whoever works it out.
fn when_it_closed(opened_at: DateTime<Utc>, interval: Interval) -> DateTime<Utc> {
    opened_at + chrono::Duration::minutes(interval.minutes())
}

/// **One decision, and everything the record needs to describe it.**
///
/// Gathered into a struct rather than passed one by one — ten loose arguments
/// is ten chances to hand over `spans_from` where `bar` was wanted, and both
/// are `&Bar` so nothing would catch it.
pub(in crate::watch::closes) struct Made<'a> {
    pub pair: &'a Pair,
    pub interval: Interval,

    /// The candle that COMPLETED the shape.
    pub bar: &'a Bar,

    /// The candle the shape STARTS on. Three back on a march, one on the rest.
    pub spans_from: &'a Bar,

    pub signal: &'a Signal,
    pub normal: Decimal,

    /// The one line he actually read on his phone.
    pub sentence: &'a str,

    /// `None` when Telegram refused it. **The bot still saw it.**
    pub sent_at: Option<DateTime<Utc>>,

    /// The setup card, waiting to go out with the two buttons on it.
    ///
    /// **It is sent here rather than with the charts**, because the buttons
    /// need the signal's row id and the row does not exist until this point.
    pub card: Option<&'a Path>,
}

/// **Where and when a refusal happened.** The same bundling, for the same
/// reason.
pub(in crate::watch::closes) struct Missed<'a> {
    pub pair: &'a Pair,
    pub interval: Interval,
    pub bar: &'a Bar,
    pub why: &'a Refused,
    pub normal: Decimal,
}

/// Writes a signal the bot decided to send.
///
/// `sent_at` is `None` when Telegram refused it. **The bot still saw it**, and
/// a signal missing from the record because a message failed would make the
/// history disagree with what the rules actually did.
///
/// **Nothing here can end the run.** A row that will not write is a gap in the
/// history; it is not a reason to stop watching his levels.
pub(in crate::watch::closes) async fn keep_signal(
    client: &reqwest::Client,
    record: Option<&Store>,
    rules_version: &str,
    made: Made<'_>,
) {
    let Some(record) = record else {
        return;
    };

    let (Ok(candle_opened_at), Ok(spans_from)) =
        (made.bar.opened_at(), made.spans_from.opened_at())
    else {
        eprintln!(
            "A signal on {} had a stamp that made no sense.",
            made.pair.symbol
        );
        return;
    };

    let signal = made.signal;
    let band = signal.standing.band();

    let row = Seen {
        // **When the bot could first know it** — see `when_it_closed`. Never
        // the moment the poll happened.
        at: when_it_closed(candle_opened_at, made.interval),
        spans_from,
        candle_opened_at,

        symbol: made.pair.symbol.clone(),
        interval: made.interval.stored().to_string(),

        shape: signal.shape.name().to_string(),
        direction: if signal.shape.is_up() { "up" } else { "down" }.into(),

        band_timeframe: band.timeframe.name().to_string(),
        band_price: band.price,
        sits: signal.standing.placing().words().to_string(),
        broke_out: signal.standing.broke_out(),
        reach: signal.reach,

        sentence: made.sentence.to_string(),
        features: with_the_band(made.bar, made.normal, &band, signal.reach),
        features_version: FEATURES_VERSION,
        rules_version: rules_version.to_string(),
        sent_at: made.sent_at,
    };

    let id = match store::sent(record, &row).await {
        // Already written. The look runs again on every poll until the next
        // candle closes, so this is the ordinary case and not a failure — and
        // it is also what stops him being asked the same question twice.
        Ok(None) => return,

        Ok(Some(id)) => {
            println!("  recorded as signal {id}");
            id
        }

        Err(trouble) => {
            eprintln!("Could not record that signal: {trouble}");
            return;
        }
    };

    // ── AND NOW ASK HIM WHAT HE THOUGHT ──
    //
    // **Only when the setup actually reached him.** A card Telegram refused is
    // still recorded, because the bot saw it — but asking "did you take it?"
    // about a setup he never received is a question with no question mark.
    if made.sent_at.is_none() {
        return;
    }

    // **The card goes out last, carrying the buttons.** He has the two charts
    // already; if this fails he has lost the card and the label, which is why
    // it is said out loud.
    if let Err(trouble) = asking::ask(client, id, made.sentence, made.card).await {
        eprintln!("Could not send the setup card: {trouble}");
    }
}

/// Writes a candle that had a shape on it and did not become a signal.
///
/// **A candle with no shape is not written down.** That is nearly every
/// candle; it would make this table larger than `candles` while saying less,
/// and it can be worked out from the candle any time. `Refused::worth_keeping`
/// is where that line is drawn.
pub(in crate::watch::closes) async fn keep_refusal(
    record: Option<&Store>,
    rules_version: &str,
    missed: Missed<'_>,
) {
    let Some(record) = record else {
        return;
    };

    if !missed.why.worth_keeping() {
        return;
    }

    let Ok(candle_opened_at) = missed.bar.opened_at() else {
        return;
    };

    let row = Turned {
        // The same moment, for the same reason: a refusal is a decision about
        // a candle, and it was knowable when that candle closed.
        at: when_it_closed(candle_opened_at, missed.interval),
        candle_opened_at,
        symbol: missed.pair.symbol.clone(),
        interval: missed.interval.stored().to_string(),
        layer: missed.why.layer().to_string(),
        why: missed.why.why(),

        // **No band, because there was no band.** A refusal at the place test
        // is the shape printing with nothing under it, and putting the nearest
        // level in anyway would record a level the rules never considered.
        features: of_the_candle(missed.bar, missed.normal),
        features_version: FEATURES_VERSION,
        rules_version: rules_version.to_string(),
    };

    match store::refused(record, &row).await {
        Ok(false) => {}
        Ok(true) => println!("  refused [{}]: {}", row.layer, row.why),
        Err(trouble) => eprintln!("Could not record that refusal: {trouble}"),
    }
}

#[cfg(test)]
mod tests {
    use super::when_it_closed;
    use chrono::{DateTime, Utc};
    use nsc_data::source::Interval;

    fn moment(text: &str) -> DateTime<Utc> {
        text.parse().expect("a real moment")
    }

    /// **The candle that opened at 14:00 on the hourly was knowable at 15:00.**
    /// Not when the poll happened to run.
    #[test]
    fn it_is_the_close_not_the_poll() {
        assert_eq!(
            when_it_closed(moment("2026-09-02T14:00:00Z"), Interval::H1),
            moment("2026-09-02T15:00:00Z")
        );
    }

    #[test]
    fn every_timeframe_lands_on_its_own_close() {
        let opened = moment("2026-09-02T12:00:00Z");

        assert_eq!(
            when_it_closed(opened, Interval::H4),
            moment("2026-09-02T16:00:00Z")
        );
        assert_eq!(
            when_it_closed(opened, Interval::Day),
            moment("2026-09-03T12:00:00Z")
        );
    }

    /// **It never lands before the candle it describes.** An `at` earlier than
    /// the candle's own open would be a signal the market had not printed yet
    /// — the one mistake in this project that makes results look better.
    #[test]
    fn it_is_never_before_the_candle_itself() {
        let opened = moment("2026-09-02T09:00:00Z");

        for interval in [Interval::H1, Interval::H4, Interval::Day, Interval::Week] {
            assert!(
                when_it_closed(opened, interval) > opened,
                "{interval:?} must close after it opened"
            );
        }
    }

    /// **The same answer whoever works it out.** Called twice a minute apart it
    /// gives the same moment, which is exactly what `Utc::now()` could not do —
    /// and why a backtest and the live bot would have disagreed.
    #[test]
    fn it_does_not_depend_on_when_it_is_asked() {
        let opened = moment("2026-09-02T14:00:00Z");

        assert_eq!(
            when_it_closed(opened, Interval::H1),
            when_it_closed(opened, Interval::H1)
        );
    }
}
