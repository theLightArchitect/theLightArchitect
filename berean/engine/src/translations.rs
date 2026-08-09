//! Side-by-side translation comparison for a reference, flagging where
//! translators genuinely diverge rather than just differ stylistically.
//! Defuses "translation bias" objections with data instead of reassurance.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CompareTranslationsQuery {
    pub reference: String,
    /// e.g. ["ESV", "NASB", "NIV", "KJV"]
    pub translations: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TranslationRendering {
    pub translation: String,
    pub text: Option<String>,
    pub found: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompareTranslationsResult {
    pub reference: String,
    pub renderings: Vec<TranslationRendering>,
    /// True only once a real divergence has been identified from actual
    /// retrieved text for at least two translations — never inferred just
    /// from which translation names were requested.
    pub notable_divergence: bool,
}

/// TODO: back each rendering with a real corpus::lookup_passage call per
/// translation, then diff the results. Until then, every rendering is
/// honestly unfound rather than a guessed paraphrase.
pub fn compare_translations(query: CompareTranslationsQuery) -> CompareTranslationsResult {
    let renderings = query
        .translations
        .into_iter()
        .map(|translation| TranslationRendering {
            translation,
            text: None,
            found: false,
        })
        .collect();
    CompareTranslationsResult {
        reference: query.reference,
        renderings,
        notable_divergence: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_claims_a_divergence_it_has_not_actually_found() {
        let result = compare_translations(CompareTranslationsQuery {
            reference: "Romans 3:25".into(),
            translations: vec!["ESV".into(), "NASB".into()],
        });
        assert!(!result.notable_divergence);
        assert!(result.renderings.iter().all(|r| !r.found));
    }
}
