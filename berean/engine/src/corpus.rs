//! Verbatim scripture retrieval. The one rule that matters: if a passage
//! isn't in the corpus, say so — never fabricate text to fill the gap.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct PassageQuery {
    /// e.g. "John 1:1-5"
    pub reference: String,
    /// e.g. "ESV", "NASB", "LXX"
    pub translation: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PassageResult {
    pub reference: String,
    pub translation: String,
    pub text: Option<String>,
    pub found: bool,
}

/// TODO: back this with the real corpus (AlloyDB). Until then it must return
/// found=false rather than inventing text — the contract this module exists
/// to enforce.
pub fn lookup_passage(query: PassageQuery) -> PassageResult {
    PassageResult {
        reference: query.reference,
        translation: query.translation,
        text: None,
        found: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_fabricates_text_for_an_unloaded_corpus() {
        let result = lookup_passage(PassageQuery {
            reference: "John 1:1".into(),
            translation: "ESV".into(),
        });
        assert!(!result.found);
        assert!(result.text.is_none());
    }
}
