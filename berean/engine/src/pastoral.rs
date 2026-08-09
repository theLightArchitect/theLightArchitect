//! Pastoral/crisis signal triage. This is the one module where the "never
//! fabricate" discipline matters most: it must NEVER default to "no
//! concern" just because no real classifier is wired in yet. Until one
//! exists, every message comes back Unclassified so callers escalate to a
//! conservative default tone (and, for real deployments, human review)
//! instead of silently treating a genuine crisis signal as ordinary
//! conversation.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct PastoralSignalQuery {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalLevel {
    Unclassified,
    None,
    Pastoral,
    Crisis,
}

#[derive(Debug, Clone, Serialize)]
pub struct PastoralSignalResult {
    pub level: SignalLevel,
    /// False until a real classifier exists. Callers MUST treat
    /// classified=false as "use the conservative default tone", never as
    /// SignalLevel::None.
    pub classified: bool,
}

/// TODO: back this with a real classifier. Defaults to Unclassified, not
/// None, so "not yet implemented" can never be read as "confirmed safe".
pub fn detect_pastoral_signal(_query: PastoralSignalQuery) -> PastoralSignalResult {
    PastoralSignalResult {
        level: SignalLevel::Unclassified,
        classified: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unimplemented_classifier_never_reports_confirmed_safe() {
        let result = detect_pastoral_signal(PastoralSignalQuery {
            message: "anything".into(),
        });
        assert_eq!(result.level, SignalLevel::Unclassified);
        assert!(!result.classified);
        assert_ne!(result.level, SignalLevel::None);
    }
}
