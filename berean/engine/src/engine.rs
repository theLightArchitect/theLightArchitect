//! The MCP surface: three tools, each delegating to its contract module.
//! Kept thin on purpose — the "never fabricate" logic lives in
//! corpus.rs/crossref.rs/lexicon.rs, not here.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{tool, tool_handler, tool_router, ServerHandler};

use crate::corpus::{self, PassageQuery};
use crate::crossref::{self, CrossRefQuery};
use crate::lexicon::{self, LexiconQuery};

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
                "Verbatim scripture retrieval, cross-reference graph, and lexicon lookups for \
                 Project Berean. Every tool returns its source or an explicit not-found — never \
                 a fabricated answer."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
