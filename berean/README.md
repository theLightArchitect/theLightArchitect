# Project Berean

An AI-powered Bible study app for anyone from a new creation in Christ to the
elders in Christ — and for the curious still on the fence.

Named for Acts 17:11: *"these were more noble... they searched the scriptures
daily to see if these things were so."* That's the design principle, not just
the name: the app shows its receipts. Every answer is traceable back to the
actual text, and the app never collapses a genuinely contested question into
one confident-sounding position.

## Structure

```
berean/
├── docs/ARCHITECTURE.md   — full architecture writeup
├── agents/                — ADK (Agent Development Kit) multi-agent system,
│                             deployed to the Gemini Enterprise Agent Platform
└── engine/                — Berean Engine: Rust MCP server owning the
                              verbatim-citation contract, cross-reference
                              graph, and lexicon lookups
```

## Status

Early scaffold. Agent topology and tool contracts are defined; corpus,
cross-reference, and lexicon data are not yet loaded (every lookup honestly
returns "not found" rather than a placeholder answer — see
`engine/src/corpus.rs`).

See `docs/ARCHITECTURE.md` for the full design, and the READMEs in `agents/`
and `engine/` for how to run each piece.
