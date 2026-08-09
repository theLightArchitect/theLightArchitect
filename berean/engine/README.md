# Berean Engine

Rust MCP server exposing ten tools: `lookup_passage`, `lookup_crossrefs`,
`lookup_lexicon`, `lookup_manuscript_variants`, `lookup_confession`,
`search_patristics`, `compare_translations`, `detect_pastoral_signal`,
`read_journal`, `write_journal`. Runs over stdio so ADK's `McpToolset` can
connect to it directly (see `../agents/berean_agents/tools/mcp_engine.py`).

Every tool follows the same discipline, adapted per domain — see
`docs/ARCHITECTURE.md` for the full table:

- Content lookups (`corpus`, `crossref`, `lexicon`, `criticism`,
  `confessions`, `patristics`, `translations`) return the data with its exact
  source, or say "not found" — never fabricate.
- `pastoral.rs` never defaults to "confirmed safe" — `classified: false`
  means "not yet classified," not "no concern."
- `journal.rs` never claims a save that didn't happen — `persisted: false`
  until real durable storage exists.

## Build & test

```bash
cargo build --release
cargo test
```

## Smoke-test the MCP handshake directly

```bash
{
  echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"smoke-test","version":"0.1"}}}'
  echo '{"jsonrpc":"2.0","method":"notifications/initialized"}'
  echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
} | ./target/release/berean-engine
```

## Status

Stub data layer — every lookup honestly returns not-found/not-classified/
not-persisted. Wiring in the real corpus (AlloyDB), curated cross-reference
dataset, lexicon, manuscript apparatus, confessional documents, and
patristic corpus is the next step; the MCP contract and tool surface are in
place, and verified end-to-end against the real ADK agents in `../agents`,
so agent development isn't blocked on data ingestion.
