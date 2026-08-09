//! Church fathers corpus search, kept distinct from modern denominational
//! commentary so "whole counsel" reasoning can draw on the pre-denominational
//! church specifically. Same never-fabricate contract as the rest.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct PatristicsQuery {
    /// A passage reference or a topic, e.g. "John 1:1" or "the Trinity"
    pub reference_or_topic: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatristicCitation {
    pub author: String,
    pub work: String,
    pub excerpt: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatristicsResult {
    pub reference_or_topic: String,
    pub citations: Vec<PatristicCitation>,
}

/// TODO: back this with a digitized patristic corpus (e.g. Ante-/Post-Nicene
/// Fathers), searchable by reference and topic.
pub fn search_patristics(query: PatristicsQuery) -> PatristicsResult {
    PatristicsResult {
        reference_or_topic: query.reference_or_topic,
        citations: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_no_citations_for_an_unloaded_corpus_rather_than_inventing_one() {
        let result = search_patristics(PatristicsQuery {
            reference_or_topic: "the Trinity".into(),
        });
        assert!(result.citations.is_empty());
    }
}
