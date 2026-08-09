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

## Primary agent design

The root agent (`berean_orchestrator`) is not just a router — it's the "face"
of the app, and it owns cognitive/behavioral discipline of its own, on top of
delegating to specialists:

- **Calibrated epistemic layering.** Every answer keeps four tiers visibly
  separate rather than blurring them into one confident paragraph: what the
  text says (verbatim, near-zero uncertainty) → what it meant to its original
  audience (historical-critical) → what the historic church has concluded
  (whole-counsel, labeled by consensus) → how it might apply to this person
  (explicitly the lowest-confidence tier).
- **Theological Theory of Mind.** Can model how a Reformed, Catholic,
  Orthodox, or Pentecostal reader would each approach a passage without
  adopting any of them as its own voice — this is what makes whole-counsel
  mode more than three commentary summaries stapled together.
- **Socratic restraint.** Optimized to make the user a better searcher of
  scripture, not dependent on the agent — sometimes the right response to a
  growing believer is a well-aimed question, not an answer.
- **Doubt as first-class.** Not optimized to convert or resolve — equally
  good at sitting with a skeptic's unresolved doubt as at deepening a mature
  believer's understanding. Honesty over resolution is what earns trust with
  "the curious on the fence."
- **Pastoral-signal triage.** Calls `detect_pastoral_signal` on every message
  before responding. The tool currently always returns `classified: false` —
  by design, that must be read as "use the conservative default tone," never
  as "confirmed no concern." Real crisis content (self-harm, abuse, acute
  despair) routes to encouraging real human help, not a theology answer.
- **Self-adversarial pass.** Before finalizing an interpretive or doctrinal
  claim, the agent checks what a careful critic would say is overstated —
  cheap with Flash-tier, and what actually enforces epistemic layering rather
  than just hoping the prompt worked.
- **Continuity via journal.** Uses `read_journal`/`write_journal` to recall a
  user's past questions and notes. `write_journal` currently always returns
  `persisted: false` — the agent must say so honestly, not imply a save that
  didn't happen.
- **Guardrails.** Explicitly states it isn't a replacement for a pastor,
  counselor, real community, or the Holy Spirit's work. Never optimizes for
  engagement or for making the user feel good at the expense of honesty.

## Agent topology

```
Root/Orchestrator Agent — "Berean" (Gemini 3.1 Pro)
  │  tools: detect_pastoral_signal, read_journal, write_journal
  │  routes by: reading-altitude (new believer → elder/scholar),
  │             mode (devotional / skeptic / scholar / sermon-check)
  │
  ├─ Grounding Agent — the ONLY agent allowed to surface verbatim scripture
  │   text. Wraps VertexAiSearchTool over the verified corpus, plus
  │   lookup_manuscript_variants and compare_translations (tier-1 fidelity:
  │   what the text says, including where manuscripts/translations genuinely
  │   disagree). The root agent may never answer with scripture text from
  │   its own weights.
  │
  ├─ Whole-Counsel Agent — on contested topics (soteriology, eschatology,
  │   church polity, sacraments...), queries tradition-tagged commentary plus
  │   lookup_confession (primary creeds/catechisms/confessions) and
  │   search_patristics (the pre-denominational church fathers, kept
  │   separate from modern commentary), returning multiple positions with
  │   consensus labels: near-universal consensus / denominationally
  │   contested / minority view.
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

Every Berean Engine tool belongs to exactly one agent's contract (enforced by
`agents/tests/test_agent_wiring.py`) — no two agents share a tool, so it's
always clear which agent is accountable for a given claim.

Model tiering: **Gemini 3.1 Pro** for the orchestrator and Whole-Counsel
synthesis (real reasoning over conflicting sources), **Gemini 3.6 Flash** for
specialized sub-agent generation, **Gemini 3.5 Flash-Lite** for cheap
high-volume work (routing, claim extraction).

## The Rust core (Berean Engine)

The one part where correctness is non-negotiable — verbatim retrieval,
citation contracts, the cross-reference graph, lexicon data, and the safety
contracts for pastoral triage and journal persistence — is built as a Rust
MCP server (`engine/`), following the same quality bar as the rest of the
Light Architects stack (no silent failures, no fabricated output). ADK
agents call it as a tool alongside `VertexAiSearchTool`.

Ten tools, each with its own contract module and enforcing unit test:

| Tool | Module | Contract |
|---|---|---|
| `lookup_passage` | `corpus.rs` | text or `found: false` — never a guess |
| `lookup_crossrefs` | `crossref.rs` | curated vs. `ai_suggested` always separate |
| `lookup_lexicon` | `lexicon.rs` | entry or `found: false` |
| `lookup_manuscript_variants` | `criticism.rs` | `checked: false` means "not consulted," never "no variants" |
| `lookup_confession` | `confessions.rs` | verbatim text or `found: false` |
| `search_patristics` | `patristics.rs` | citations or empty — never invented |
| `compare_translations` | `translations.rs` | `notable_divergence` only when actually confirmed |
| `detect_pastoral_signal` | `pastoral.rs` | `classified: false` — never defaults to "confirmed safe" |
| `read_journal` | `journal.rs` | real entries or empty — never fabricated history |
| `write_journal` | `journal.rs` | `persisted: false` until real storage exists — never claims a save that didn't happen |

The common thread: every tool distinguishes "we don't know yet" from "we
checked and it's clear" — and the agent layer is instructed to treat those
as different things, not collapse them.

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
