//! What the watcher carries, and what has to survive the line dropping.

use nsc_core::levels::{Pair, Watch};

use super::{closes, pulse, reload, resumed};

/// Everything being watched for one pair.
pub(crate) struct Watching {
    pub pair: Pair,
    pub watch: Watch,
}

/// What has to outlive the socket.
///
/// **Rebuilt on every reconnect, a dropped line would re-announce every zone
/// price is already sitting in and forget which candles it had reported.** It
/// is one struct rather than five arguments because it travels together and
/// always has — five of them was `too_many_arguments` with the lint switched
/// off, which is the compiler being told to stop noticing.
pub(crate) struct Kit {
    pub closes: closes::Closes,
    pub awake: resumed::Awake,
    pub pulse: pulse::Pulse,
    pub files: reload::Files,
}

impl Kit {
    pub fn new() -> Self {
        Kit {
            closes: closes::Closes::new(),
            awake: resumed::Awake::new(),
            pulse: pulse::Pulse::new(),
            files: reload::Files::look(),
        }
    }
}
