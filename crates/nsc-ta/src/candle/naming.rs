//! Which name a shape gets, and why that is a running order rather than a set.

use super::{Named, Rules, Shape};

impl Shape {
    /// What this candle is called.
    ///
    /// **One name, decided by testing the tightest rule first.**
    ///
    /// Real candles sit in more than one family at once — a dragonfly doji is
    /// also a long lower wick, and a belt-hold is also a long body. Answering
    /// with every true name would be honest and useless: the caller would have
    /// to pick anyway, and every caller would pick differently.
    ///
    /// So the order below IS the meaning, and it runs from the strictest
    /// condition to the loosest:
    ///
    /// ```text
    ///     almost no body      the body test is the strongest thing about it
    ///     almost all body     likewise, the other way
    ///     a long wick one way price went looking and was refused
    ///     long wicks both ways nobody won
    ///     a small body        neither side finished in charge
    ///     none of the above   most candles
    /// ```
    pub fn named(&self, rules: &Rules) -> Named {
        if self.body <= rules.body.doji {
            return self.doji_kind(rules);
        }

        if self.body >= rules.body.strong {
            return self.strong_kind(rules);
        }

        if let Some(rejection) = self.rejection_kind(rules) {
            return rejection;
        }

        if self.body <= rules.body.small
            && self.upper >= rules.wick.long
            && self.lower >= rules.wick.long
        {
            return Named::HighWave;
        }

        if self.body <= rules.body.small {
            return Named::SpinningTop;
        }

        Named::Plain
    }

    /// Almost no body. Which one depends entirely on where the wicks are.
    fn doji_kind(&self, rules: &Rules) -> Named {
        let long_up = self.upper >= rules.wick.long;
        let long_down = self.lower >= rules.wick.long;

        // **`stub`, not `missing`.** A dragonfly's short end is short beside a
        // tail of 0.90, not beside a marubozu's nothing. Judged by `missing`,
        // the clearest real dragonfly and gravestone in three years both came
        // back as plain dojis.
        let no_up = self.upper <= rules.wick.stub;
        let no_down = self.lower <= rules.wick.stub;

        match (long_down && no_up, long_up && no_down, long_up && long_down) {
            (true, _, _) => Named::DragonflyDoji,
            (_, true, _) => Named::GravestoneDoji,
            (_, _, true) => Named::LongLeggedDoji,
            _ => Named::Doji,
        }
    }

    /// Almost all body. Which one depends on which END has no wick.
    fn strong_kind(&self, rules: &Rules) -> Named {
        let no_up = self.upper <= rules.wick.missing;
        let no_down = self.lower <= rules.wick.missing;

        if no_up && no_down {
            return Named::Marubozu;
        }

        // **Which wick sits at the open depends on which way it closed.** A
        // candle that closed up opened at the bottom of its body, so the LOWER
        // wick is the one at its open. Get this backwards and every bullish
        // belt-hold is read as a bearish one.
        let (at_open, at_close) = if self.up {
            (no_down, no_up)
        } else {
            (no_up, no_down)
        };

        match (at_open, at_close) {
            (true, _) => Named::BeltHold,
            (_, true) => Named::ClosingMarubozu,
            _ => Named::LongBody,
        }
    }

    /// A long wick one way with a small body at the far end, or nothing.
    ///
    /// **The body must be small AND the tail must dwarf it.** Either test on
    /// its own lets through candles nobody would call a rejection: a small
    /// body with two short wicks, or a long tail under a body just as long.
    fn rejection_kind(&self, rules: &Rules) -> Option<Named> {
        if self.body > rules.body.small {
            return None;
        }

        let enough = self.body * rules.rejection.tail_to_body;

        if self.lower >= enough && self.upper <= rules.rejection.nose {
            return Some(Named::LongLowerWick);
        }

        if self.upper >= enough && self.lower <= rules.rejection.nose {
            return Some(Named::LongUpperWick);
        }

        None
    }
}
