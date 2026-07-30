# Berean Engine

Rust MCP server exposing three tools: `lookup_passage`, `lookup_crossrefs`,
`lookup_lexicon`. Runs over stdio so ADK's `McpToolset` can connect to it
directly (see `../agents/berean_agents/tools/mcp_engine.py`).

Every tool follows the same contract: return the data with its exact source,
or say "not found" — never fabricate. See `src/corpus.rs` for the enforcing
test.

## Build & test

```bash
cargo build --release
cargo test
```

## Status

Stub data layer — every lookup honestly returns not-found. Wiring in the
real corpus (AlloyDB), curated cross-reference dataset, and lexicon data is
the next step; the MCP contract and tool surface are in place so agents can
already be developed against it.
