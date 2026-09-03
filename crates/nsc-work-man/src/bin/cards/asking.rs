//! **Recording a previewed setup, so it can carry the two buttons.**
//!
//! The buttons hold a signal's row id — they cannot exist without a row. So
//! the only way to see them before the bot next finds a live setup is to
//! record one, and that is what this does.
//!
//! **The row is honest.** A real shape, at a real level he drew, on a candle
//! that really closed. What it is not is NEW: `candle_opened_at` says when it
//! printed, which may be days ago, and `rules_version` says `preview` so these
//! never mix with the live ones in a count.

use chrono::Utc;
use nsc_core::candle::Bar;
use nsc_core::levels::Pair;
use nsc_data::source::Interval;
use nsc_data::store;

/// Records the signal and sends the buttons under it.
///
/// **Nothing here can stop the preview.** No database, or a row that will not
/// write, costs the buttons — not the pictures he asked for.
pub(super) async fn ask_him(
    client: &reqwest::Client,
    pair: &Pair,
    signal: &nsc_strategy::Signal,
    history: &[&Bar],
    sentence: &str,
) {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        println!("    (no DATABASE_URL, so no buttons)");
        return;
    };

    let Ok(record) = store::open(&url).await else {
        println!("    (could not open the record, so no buttons)");
        return;
    };

    let Some(last) = history.last() else { return };
    let Ok(opened_at) = last.opened_at() else {
        return;
    };

    let band = signal.standing.band();
    let now = Utc::now();

    let row = store::Seen {
        // The candle closed an hour after it opened. Same sum the live path
        // uses — see `recording::writing`.
        at: opened_at + chrono::Duration::minutes(Interval::H1.minutes()),
        spans_from: opened_at,
        candle_opened_at: opened_at,
        symbol: pair.symbol.clone(),
        interval: Interval::H1.stored().to_string(),
        shape: signal.shape.name().to_string(),
        direction: if signal.shape.is_up() { "up" } else { "down" }.into(),
        band_timeframe: band.timeframe.name().to_string(),
        band_price: band.price,
        sits: signal.standing.placing().words().to_string(),
        broke_out: signal.standing.broke_out(),
        reach: signal.reach,
        sentence: sentence.to_string(),
        features: serde_json::json!({ "from": "the charts previewer" }),
        features_version: 1,
        rules_version: "preview".into(),
        sent_at: Some(now),
    };

    // **Recorded if it is new, looked up if it is not.**
    //
    // The bot refuses a repeat, and rightly — one shape, one candle, one zone,
    // one card. A previewer is the opposite: showing him a setup again is the
    // whole job, and the buttons still need the id.
    let id = match store::sent(&record, &row).await {
        Ok(Some(id)) => Some(id),

        Ok(None) => store::already(
            &record,
            &row.symbol,
            &row.interval,
            row.band_price,
            row.candle_opened_at,
        )
        .await
        .unwrap_or_default(),

        Err(trouble) => {
            println!("    (could not record it: {trouble})");
            return;
        }
    };

    let Some(id) = id else {
        println!("    (no row to hang the buttons on)");
        return;
    };

    match nsc_work_man::watch::ask_about(client, id).await {
        Ok(()) => println!("    buttons on signal {id}"),
        Err(trouble) => println!("    (the buttons did not send: {trouble})"),
    }
}
