mod confessions;
mod corpus;
mod criticism;
mod crossref;
mod engine;
mod journal;
mod lexicon;
mod pastoral;
mod patristics;
mod translations;

use engine::BereanEngine;
use rmcp::transport::stdio;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = BereanEngine::default().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
