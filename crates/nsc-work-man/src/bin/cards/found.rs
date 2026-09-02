//! The setup card — rung 3, drawn without waiting for one to print.
//!
//! **The candles are real**: gold's daily, 19 and 20 August 2026 — the two off
//! his own screenshot, the same pair `nsc-strategy` is tested against.
//!
//! **The zone is placed for the picture**, because a real one has to be where
//! price actually was. That is said out loud here rather than left to be
//! assumed.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::candle::{Bar, normal_candle};
use nsc_core::levels::{Band, Pair, Timeframe};
use nsc_strategy::{look, reasons};
use nsc_ta::pattern;
use nsc_work_man::places::{OWNER, PATTERNS, PREVIEW, STRATEGY};
use nsc_work_man::{card, telegram};
use rust_decimal::Decimal;

fn d(text: &str) -> Decimal {
    text.parse().expect("a price")
}

fn bar(stamp: &str, open: &str, high: &str, low: &str, close: &str) -> Bar {
    Bar {
        datetime: stamp.into(),
        open: d(open),
        high: d(high),
        low: d(low),
        close: d(close),
    }
}

pub async fn setup(client: &reqwest::Client, tier: Option<&str>) -> Result<()> {
    let rules = nsc_strategy::load(Path::new(STRATEGY))
        .with_context(|| format!("could not read {STRATEGY}"))?;
    let patterns =
        pattern::load(Path::new(PATTERNS)).with_context(|| format!("could not read {PATTERNS}"))?;

    // His own gold. A push of 1.9x a normal day with 87% body, then a tail of
    // 65 points under a body of five.
    let bars = [
        bar(
            "2026-08-19 00:00:00",
            "4344.53",
            "4524.36",
            "4324.71",
            "4517.78",
        ),
        bar(
            "2026-08-20 00:00:00",
            "4520.67",
            "4541.06",
            "4450.71",
            "4515.78",
        ),
    ];

    let history: Vec<&Bar> = bars.iter().collect();

    // Normal is handed in rather than averaged, because two candles cannot
    // give a fourteen-candle average. It is the real one from that week.
    let normal = normal_candle(&history, 14).unwrap_or_else(|| d("104.27462"));

    // **One run, both tiers**, so each look can be checked without waiting for
    // the market to print it.
    let (bands, normal) = match tier {
        // Within half a band of it: the pin's low of 4450.71 sits 10.71 above
        // a band topping at 4440, and half of a 40-point band is 20.
        Some("close") => (
            vec![Band {
                timeframe: Timeframe::Daily,
                price: d("4420"),
                top: d("4440"),
                bottom: d("4400"),
            }],
            normal,
        ),

        // The pin's low is 4450.71, so a weekly band around 4470 holds it.
        _ => (
            vec![Band {
                timeframe: Timeframe::Weekly,
                price: d("4470"),
                top: d("4508"),
                bottom: d("4432"),
            }],
            normal,
        ),
    };

    let signal = look(&history, &bands, normal, &patterns, &rules).map_err(|why| {
        anyhow::anyhow!(
            "his own gold should be a signal at that zone — refused at the {} layer: {}",
            why.layer(),
            why.why()
        )
    })?;

    let pair = Pair {
        symbol: "XAU/USD".into(),
        digits: 2,
        nightly_break_minutes: 60,
        approach_share: None,
        levels: Vec::new(),
    };

    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from(PREVIEW).join("setup.png");
    let picture = card::setup(&signal, &pair, &history, "1d", &stamp, &out)?;

    println!(
        "{}\n",
        reasons::sentence(&signal, &pair.symbol, "1d", pair.digits)
    );

    telegram::send_to(
        client,
        &OWNER.to_string(),
        &[&picture],
        "🎨 <b>Preview.</b> This is a rung 3 setup card — real candles, a zone \
         placed for the picture.",
    )
    .await?;

    println!("Drawn to {} and sent.", picture.display());

    Ok(())
}
