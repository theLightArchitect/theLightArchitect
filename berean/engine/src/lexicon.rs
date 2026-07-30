//! Original-language lookups: Strong's number, morphology, semantic range.
//! Same contract as corpus.rs: say "not found", don't guess at a gloss.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct LexiconQuery {
    /// The word as it appears in context, or a Strong's number (e.g. "G26").
    pub word_or_strongs: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LexiconResult {
    pub query: String,
    pub strongs_number: Option<String>,
    pub morphology: Option<String>,
    pub gloss: Option<String>,
    pub found: bool,
}

/// TODO: back this with a real lexicon dataset (e.g. digitized Strong's +
/// morphological tagging).
pub fn lookup_lexicon(query: LexiconQuery) -> LexiconResult {
    LexiconResult {
        query: query.word_or_strongs.clone(),
        strongs_number: None,
        morphology: None,
        gloss: None,
        found: false,
    }
}
