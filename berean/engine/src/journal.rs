//! Per-user study journal / walk-timeline. Stubbed with explicit
//! in-memory-only semantics: writes report persisted=false rather than
//! silently pretending to have saved something durable. AlloyDB-backed
//! storage is not wired in yet.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ReadJournalQuery {
    pub user_id: String,
    /// Optional passage reference to filter by.
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JournalEntry {
    pub id: String,
    pub reference: Option<String>,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadJournalResult {
    pub user_id: String,
    pub entries: Vec<JournalEntry>,
}

/// TODO: back with AlloyDB. Returns an empty history, never a fabricated one.
pub fn read_journal(query: ReadJournalQuery) -> ReadJournalResult {
    ReadJournalResult {
        user_id: query.user_id,
        entries: Vec::new(),
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct WriteJournalQuery {
    pub user_id: String,
    pub reference: Option<String>,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WriteJournalResult {
    /// False until AlloyDB-backed storage exists — callers must not tell the
    /// user their note was saved when it wasn't.
    pub persisted: bool,
}

/// TODO: back with AlloyDB. Must keep reporting persisted=false until real
/// durable storage exists — never claim a save that didn't happen.
pub fn write_journal(_query: WriteJournalQuery) -> WriteJournalResult {
    WriteJournalResult { persisted: false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_never_claims_persistence_before_a_real_store_exists() {
        let result = write_journal(WriteJournalQuery {
            user_id: "u1".into(),
            reference: Some("Psalm 23:1".into()),
            note: "test".into(),
        });
        assert!(!result.persisted);
    }

    #[test]
    fn read_returns_empty_history_rather_than_fabricated_entries() {
        let result = read_journal(ReadJournalQuery {
            user_id: "u1".into(),
            reference: None,
        });
        assert!(result.entries.is_empty());
    }
}
