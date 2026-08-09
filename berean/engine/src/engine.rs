//! The MCP surface: each tool delegates to its contract module. Kept thin
//! on purpose — the "never fabricate" logic lives in the per-domain modules
//! (corpus.rs, crossref.rs, lexicon.rs, criticism.rs, confessions.rs,
//! patristics.rs, translations.rs, pastoral.rs, journal.rs), not here.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{tool, tool_handler, tool_router, ServerHandler};

use crate::confessions::{self, ConfessionQuery};
use crate::corpus::{self, PassageQuery};
use crate::criticism::{self, VariantQuery};
use crate::crossref::{self, CrossRefQuery};
use crate::journal::{self, ReadJournalQuery, WriteJournalQuery};
use crate::lexicon::{self, LexiconQuery};
use crate::pastoral::{self, PastoralSignalQuery};
use crate::patristics::{self, PatristicsQuery};
use crate::translations::{self, CompareTranslationsQuery};

#[derive(Clone)]
pub struct BereanEngine {
    tool_router: ToolRouter<Self>,
}

impl Default for BereanEngine {
    fn default() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl BereanEngine {
    #[tool(
        description = "Retrieve scripture text verbatim by reference and translation. Returns found=false rather than guessing when the passage isn't in the corpus."
    )]
    async fn lookup_passage(&self, Parameters(query): Parameters<PassageQuery>) -> String {
        serde_json::to_string(&corpus::lookup_passage(query)).unwrap_or_default()
    }

    #[tool(
        description = "Look up curated and AI-suggested cross-references for a passage, kept clearly separate."
    )]
    async fn lookup_crossrefs(&self, Parameters(query): Parameters<CrossRefQuery>) -> String {
        serde_json::to_string(&crossref::lookup_crossrefs(query)).unwrap_or_default()
    }

    #[tool(
        description = "Look up Strong's number, morphology, and semantic range for a word or Strong's number."
    )]
    async fn lookup_lexicon(&self, Parameters(query): Parameters<LexiconQuery>) -> String {
        serde_json::to_string(&lexicon::lookup_lexicon(query)).unwrap_or_default()
    }

    #[tool(
        description = "Look up textual-critical manuscript variants for a reference. checked=false means the apparatus hasn't been consulted yet — never read it as 'no variants exist'."
    )]
    async fn lookup_manuscript_variants(
        &self,
        Parameters(query): Parameters<VariantQuery>,
    ) -> String {
        serde_json::to_string(&criticism::lookup_manuscript_variants(query)).unwrap_or_default()
    }

    #[tool(
        description = "Look up a primary confessional document (creed, catechism, confession) by name/section and optional tradition tag."
    )]
    async fn lookup_confession(&self, Parameters(query): Parameters<ConfessionQuery>) -> String {
        serde_json::to_string(&confessions::lookup_confession(query)).unwrap_or_default()
    }

    #[tool(
        description = "Search the church fathers corpus by reference or topic, kept separate from modern denominational commentary."
    )]
    async fn search_patristics(&self, Parameters(query): Parameters<PatristicsQuery>) -> String {
        serde_json::to_string(&patristics::search_patristics(query)).unwrap_or_default()
    }

    #[tool(
        description = "Compare a passage across multiple translations side by side, flagging only genuine, retrieval-confirmed divergences."
    )]
    async fn compare_translations(
        &self,
        Parameters(query): Parameters<CompareTranslationsQuery>,
    ) -> String {
        serde_json::to_string(&translations::compare_translations(query)).unwrap_or_default()
    }

    #[tool(
        description = "Triage a message for pastoral or crisis signal. Always returns classified=false until a real classifier is wired in — callers must treat that as 'use the conservative default tone', never as 'confirmed no concern'."
    )]
    async fn detect_pastoral_signal(
        &self,
        Parameters(query): Parameters<PastoralSignalQuery>,
    ) -> String {
        serde_json::to_string(&pastoral::detect_pastoral_signal(query)).unwrap_or_default()
    }

    #[tool(
        description = "Read a user's study journal / walk-timeline, optionally filtered by passage reference."
    )]
    async fn read_journal(&self, Parameters(query): Parameters<ReadJournalQuery>) -> String {
        serde_json::to_string(&journal::read_journal(query)).unwrap_or_default()
    }

    #[tool(
        description = "Write a note to a user's study journal. Always returns persisted=false until durable storage is wired in — never claim a save that didn't happen."
    )]
    async fn write_journal(&self, Parameters(query): Parameters<WriteJournalQuery>) -> String {
        serde_json::to_string(&journal::write_journal(query)).unwrap_or_default()
    }
}

#[tool_handler]
impl ServerHandler for BereanEngine {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: rmcp::model::Implementation {
                name: "berean-engine".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                ..Default::default()
            },
            instructions: Some(
                "Verbatim scripture retrieval, cross-reference graph, lexicon, manuscript \
                 variants, confessions, patristics, translation comparison, pastoral-signal \
                 triage, and journal access for Project Berean. Every tool returns its source \
                 or an explicit not-found/not-classified/not-persisted — never a fabricated \
                 answer or a false claim of safety or durability."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
