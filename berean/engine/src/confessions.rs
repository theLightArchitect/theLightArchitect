//! Primary confessional documents — creeds, catechisms, confessions — tagged
//! by tradition. Returns the actual text plus its tradition tag; never a
//! paraphrase presented as the genuine article.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ConfessionQuery {
    /// e.g. "Nicene Creed", "Westminster Shorter Catechism Q&A 1", "Trent Session 6"
    pub document_and_section: String,
    /// Optional tradition filter, e.g. "Reformed", "Catholic", "Orthodox", "Wesleyan"
    pub tradition: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfessionResult {
    pub document_and_section: String,
    pub tradition: Option<String>,
    pub text: Option<String>,
    pub found: bool,
}

/// TODO: back this with digitized primary confessional documents. Same
/// contract as corpus.rs: found=false, never a guess at the wording.
pub fn lookup_confession(query: ConfessionQuery) -> ConfessionResult {
    ConfessionResult {
        document_and_section: query.document_and_section,
        tradition: query.tradition,
        text: None,
        found: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_fabricates_confessional_text_for_an_unloaded_dataset() {
        let result = lookup_confession(ConfessionQuery {
            document_and_section: "Nicene Creed".into(),
            tradition: None,
        });
        assert!(!result.found);
        assert!(result.text.is_none());
    }
}
