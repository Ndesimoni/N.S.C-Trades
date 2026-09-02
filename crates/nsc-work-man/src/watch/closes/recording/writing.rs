//! Turning a decision into a row, and never letting that end the run.

use chrono::{DateTime, Utc};
use nsc_core::candle::Bar;
use nsc_core::levels::Pair;
use nsc_data::source::Interval;
use nsc_data::store::{self, Seen, Store, Turned};
use nsc_strategy::{Refused, Signal};
use rust_decimal::Decimal;

use super::features::{FEATURES_VERSION, of_the_candle, with_the_band};

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
        // **When the bot could first KNOW it.** On a candlestick shape that is
        // the close of the candle that completed it — which is the stamp of
        // the NEXT candle's open, but the close is what it acted on.
        at: Utc::now(),
        spans_from,
        candle_opened_at,

        symbol: made.pair.symbol.clone(),
        interval: made.interval.stored().to_string(),

        shape: signal.shape.name().to_string(),
        direction: if signal.shape.is_up() { "up" } else { "down" }.into(),

        band_timeframe: band.timeframe.name().to_string(),
        band_price: band.price,
        placing: signal.standing.placing().words().to_string(),
        broke_out: signal.standing.broke_out(),
        reach: signal.reach,

        sentence: made.sentence.to_string(),
        features: with_the_band(made.bar, made.normal, &band, signal.reach),
        features_version: FEATURES_VERSION,
        rules_version: rules_version.to_string(),
        sent_at: made.sent_at,
    };

    match store::sent(record, &row).await {
        // Already written. The look runs again on every poll until the next
        // candle closes, so this is the ordinary case and not a failure.
        Ok(false) => {}
        Ok(true) => println!("  recorded: {}", row.sentence),
        Err(trouble) => eprintln!("Could not record that signal: {trouble}"),
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
        at: Utc::now(),
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
