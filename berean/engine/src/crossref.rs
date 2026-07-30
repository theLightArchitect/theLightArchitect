//! Cross-reference constellation lookups. Curated (human-verified) edges and
//! AI-suggested edges are always returned as separate lists — never merged —
//! so a client can't accidentally present a model guess as an established
//! typological link.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CrossRefQuery {
    pub reference: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrossRefResult {
    pub reference: String,
    pub curated: Vec<String>,
    pub ai_suggested: Vec<String>,
}

/// TODO: back `curated` with a digitized cross-reference dataset and
/// `ai_suggested` with embedding-similarity search over the corpus.
pub fn lookup_crossrefs(query: CrossRefQuery) -> CrossRefResult {
    CrossRefResult {
        reference: query.reference,
        curated: Vec::new(),
        ai_suggested: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curated_and_ai_suggested_never_share_entries_when_unloaded() {
        let result = lookup_crossrefs(CrossRefQuery {
            reference: "Exodus 12:1-13".into(),
        });
        assert!(result.curated.is_empty());
        assert!(result.ai_suggested.is_empty());
    }
}
