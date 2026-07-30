# Architecture

## Design principle

Berean's differentiator against generic "AI Bible chat" apps: **verification-first,
not answer-first**. The app never states a doctrinal claim without showing the
primary text, and on genuinely contested questions it presents the major
positions side by side with consensus labels instead of picking one. This is
enforced structurally, not just by prompting — see "Grounding enforcement" below.

## Platform

Built on Google Cloud's **Gemini Enterprise Agent Platform** (the 2026 rebrand
of Vertex AI, with Agentspace folded in — existing Vertex AI APIs/SDKs are
unchanged, just under new branding).

- **ADK (Agent Development Kit)** — defines the multi-agent hierarchy below.
- **Agent Engine** — managed agent runtime: sessions, long-term per-user
  memory (study history / "walk timeline"), scaling.
- **Vertex AI Search** — managed RAG layer over the verified, tradition-tagged
  corpus (translations, commentary, lexicons, secular/historical-critical
  scholarship).
- **Check Grounding API** — scores every generated claim against retrieved
  context before it's shown; below threshold, regenerate or fall back to
  "here's what the text says, I can't confidently interpret further."
- **AlloyDB (with pgvector/AlloyDB AI)** — one store for relational data
  (users, study plans, group threads, curated cross-reference edges) and
  embeddings. Corpus-plus-commentary scale doesn't need a separate vector DB;
  that's an optimization for later if it's ever needed.

## Agent topology

```
Root/Orchestrator Agent (Gemini 3.1 Pro)
  ├─ routes by: reading-altitude (new believer → elder/scholar),
  │             mode (devotional / skeptic / scholar / sermon-check)
  │
  ├─ Grounding Agent — the ONLY agent allowed to surface verbatim scripture
  │   text. Wraps VertexAiSearchTool over the verified corpus. The root
  │   agent may never answer with scripture text from its own weights.
  │
  ├─ Whole-Counsel Agent — on contested topics (soteriology, eschatology,
  │   church polity, sacraments...), queries tradition-tagged commentary and
  │   returns multiple positions with consensus labels:
  │   near-universal consensus / denominationally contested / minority view.
  │
  ├─ Skeptic-Mode Agent — no faith presupposed; engages objections
  │   (apparent contradictions, textual history, science, problem of evil)
  │   honestly, citing secular and faith-based scholarship alike.
  │
  ├─ Cross-Reference Agent — queries the constellation graph via the Berean
  │   Engine (MCP). Curated (human-verified) and AI-suggested edges are
  │   always kept separate, never blurred together.
  │
  ├─ Lexicon Agent — Strong's/morphology/semantic-range lookups, via the
  │   Berean Engine (MCP).
  │
  └─ Sermon Fact-Checker Agent — extracts scriptural claims from a transcript
      (cheap high-throughput extraction), fans each claim out to the
      Grounding Agent, reports Accurate / Misquoted / Out-of-context /
      Reference-not-found.
```

Model tiering: **Gemini 3.1 Pro** for the orchestrator and Whole-Counsel
synthesis (real reasoning over conflicting sources), **Gemini 3.6 Flash** for
specialized sub-agent generation, **Gemini 3.5 Flash-Lite** for cheap
high-volume work (routing, claim extraction).

## The Rust core (Berean Engine)

The one part where correctness is non-negotiable — verbatim retrieval,
citation contracts, the cross-reference graph, lexicon data — is built as a
Rust MCP server (`engine/`), following the same quality bar as the rest of
the Light Architects stack (no silent failures, no fabricated output). ADK
agents call it as a tool alongside `VertexAiSearchTool`.

Contract: every lookup either returns text/data with its exact source, or an
explicit "not found" — never a guess. `engine/src/corpus.rs` has a unit test
enforcing this for the passage lookup.

## Deployment shape

- **Agent Engine** — agent runtime.
- **Cloud Run** — product backend/API gateway, and the Berean Engine MCP
  server.
- **AlloyDB** — relational + vector.
- Per-agent IAM service accounts, least privilege (e.g. the Sermon
  Fact-Checker gets a scoped web-fetch tool, nothing else).
- Cloud Trace/Logging + Vertex's agent evaluation tools for continuous
  groundedness monitoring, run in CI given the stakes of misquoting scripture.

## Deferred / not yet built

- Corpus ingestion (translations, commentary tagged by tradition, secular
  scholarship, lexicon data) — nothing is loaded yet.
- The curated cross-reference dataset and the embedding-similarity path for
  AI-suggested edges.
- Client apps (web/mobile) — not scaffolded yet.
- Offline-first / on-device path (Tier 3 idea from initial brainstorm) —
  worth keeping in mind for the AlloyDB/engine split, not a v1 concern.
