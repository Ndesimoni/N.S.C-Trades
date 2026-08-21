//! Lining the big candles up against the small ones, and what that proves.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use super::Lined;

/// Line every big candle up against the small ones underneath it.
///
/// **This asks the feed where its own boundaries are, rather than trusting
/// what it says.** A daily candle's open IS an hourly candle's open — the same
/// tick, written down twice — so the hour that shares the number is the hour
/// the day began.
///
/// Nothing here works a boundary out by arithmetic. That is the mistake this
/// whole check exists to catch: guessing wrong reads a candle before the
/// market printed it, and that does not error, it just makes results look
/// better.
pub fn line_up(big: &[Bar], small: &[Bar]) -> Vec<Lined> {
    big.iter()
        .map(|one| Lined {
            big: one.datetime.clone(),
            started: small
                .iter()
                .filter(|under| under.open == one.open)
                .map(|under| under.datetime.clone())
                .collect(),
            nearest: nearest_to(one.open, small),
        })
        .collect()
}

/// The small candle whose open came closest, and by how much.
///
/// **For when nothing matches exactly.** That answer is worth seeing: a miss
/// by 0.02 means the boundary is right and the feed rounds differently; a miss
/// by 40 means the boundary is somewhere else entirely.
fn nearest_to(open: Decimal, small: &[Bar]) -> Option<(String, Decimal)> {
    small
        .iter()
        .map(|under| (under.datetime.clone(), (under.open - open).abs()))
        .min_by(|a, b| a.1.cmp(&b.1))
}

/// How many candles were able to vote at all.
///
/// **A candle that matched two smaller ones has no vote.** It is not evidence
/// against an answer, it is simply silent — a quiet market opens two hours on
/// the same number, and taking the first would be a guess wearing a
/// measurement's clothes.
pub fn voted(lined: &[Lined]) -> usize {
    lined.iter().filter(|one| one.started.len() == 1).count()
}

/// The one answer every voting candle gave, if they all gave the same one.
///
/// `speak` turns a matched stamp into the thing being measured — the hour for
/// a day, the weekday for a week.
///
/// **This does NOT decide whether the evidence is enough.** It answers "did
/// the ones that spoke agree", and the caller must ask `voted` as well. The
/// first version did not, and on EUR/USD it printed EVERY CANDLE AGREES off
/// two votes out of six — which is exactly the overstatement this whole
/// folder is supposed to prevent.
pub fn agreed_on(lined: &[Lined], speak: impl Fn(&str) -> Option<String>) -> Option<String> {
    let mut said = lined
        .iter()
        .filter(|one| one.started.len() == 1)
        .filter_map(|one| speak(&one.started[0]));

    let first = said.next()?;

    said.all(|one| one == first).then_some(first)
}

/// The hour of the day a stamp falls on, as `21:15`.
pub fn hour_of(stamp: &str) -> Option<String> {
    stamp.split_once(' ').map(|(_, time)| time[..5].to_string())
}

/// The weekday a stamp falls on, as `Monday`.
///
/// **For the week, the hour on the stamp means nothing.** A daily candle
/// carries a date and no time, so every one of them reads 00:00 — and the
/// first version dutifully reported that the week starts at 00:00 UTC, which
/// is not a fact about IBKR, it is a fact about the stamp.
pub fn weekday_of(stamp: &str) -> Option<String> {
    let day = stamp.split_once(' ')?.0;

    chrono::NaiveDate::parse_from_str(day, "%Y-%m-%d")
        .ok()
        .map(|date| date.format("%A").to_string())
}
