//! Textual-critical apparatus lookups: manuscript variants for a reference.
//! Same contract as corpus.rs — report what's known, or say it isn't known
//! yet, but never smooth over a variant to make the text look more uniform
//! than it actually is in the manuscript tradition.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct VariantQuery {
    /// e.g. "Mark 16:9-20", "John 7:53-8:11"
    pub reference: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Variant {
    /// e.g. "Sinaiticus, Vaticanus" — which manuscripts attest this reading.
    pub witnesses: String,
    pub reading: String,
    pub significance: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VariantResult {
    pub reference: String,
    pub variants: Vec<Variant>,
    /// True once this reference has actually been checked against a real
    /// critical apparatus (NA28/UBS5, BHS, DSS). False means "no data" —
    /// it must NEVER be read as "confirmed variant-free".
    pub checked: bool,
}

/// TODO: back this with a digitized critical apparatus. `checked: false`
/// is the whole point of this module: it's the difference between "we
/// haven't looked" and "we looked and found nothing."
pub fn lookup_manuscript_variants(query: VariantQuery) -> VariantResult {
    VariantResult {
        reference: query.reference,
        variants: Vec::new(),
        checked: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unchecked_apparatus_is_never_presented_as_variant_free() {
        let result = lookup_manuscript_variants(VariantQuery {
            reference: "Mark 16:9-20".into(),
        });
        assert!(!result.checked);
        assert!(result.variants.is_empty());
    }
}
