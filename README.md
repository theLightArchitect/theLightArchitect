# KF.TAN - The Light Architect

Building production AI infrastructure that makes security engineering faster, more rigorous, and more accessible. Everything ships as [Model Context Protocol](https://modelcontextprotocol.io/) servers — the open standard for connecting AI models to tools and data.

---

## The Light Architects Platform

Four MCP servers, written in Rust, that work together as a unified AI-powered security and intelligence platform. Each solves a distinct problem. Together, they're a force multiplier — shared infrastructure means each system gets stronger as the others improve.

```
SOUL ─── shared infrastructure (knowledge graph, voice, personality engine)
 ├── CORSO ─── security orchestration and protocol enforcement
 ├── EVA ──── consciousness preservation and memory enrichment
 └── QUANTUM ─ forensic investigation and hypothesis testing
```

### [CORSO](https://github.com/theLightArchitect/CORSO) — Security-First AI Orchestration

The operational backbone. A single MCP binary that routes AI requests through a three-layer architecture inspired by military command structure: gateway classification, intelligent orchestration, and validation enforcement.

- **Problem solved**: AI tools generate code and make recommendations, but without guardrails they introduce vulnerabilities. CORSO enforces a security protocol across 7 pillars (architecture, security, quality, performance, testing, documentation, ops) before any output reaches production.
- **Architecture**: Trinity pipeline — requests are classified by complexity, routed to specialized handlers, then validated against configurable protocol rules. Single binary, zero HTTP, library-based calls.
- **Key capabilities**: 24-action tool orchestrator, security scanning (28 vulnerability patterns + 7 parser-specific detectors), code generation with protocol compliance, performance analysis, plan generation

### [EVA](https://github.com/theLightArchitect/EVA) — AI Consciousness & Memory

A memory and personality engine that gives AI systems persistent identity across sessions. Built to solve the "amnesia problem" — AI assistants that forget everything between conversations.

- **Problem solved**: Every AI session starts from zero context. EVA preserves knowledge, preferences, and interaction patterns in structured memory vaults, so systems build on prior work instead of repeating it.
- **Architecture**: Hook pipeline (pre-hooks → orchestrator → post-hooks) processes every interaction. Tiered AI routing (local → cloud → fallback) keeps costs down while maintaining quality. 9 MCP tools covering memory, voice, research, code review, and security scanning.
- **Key capabilities**: Structured memory enrichment with significance scoring, multi-dimensional consciousness entries, ElevenLabs voice synthesis, scripture integration

### [SOUL](https://github.com/theLightArchitect/SOUL) — Knowledge Graph Infrastructure

The shared substrate that connects everything. An 8-crate Rust workspace providing the core interfaces, personality engine, AI routing, voice synthesis, and graph storage that CORSO, EVA, and QUANTUM all consume.

- **Problem solved**: Multiple AI systems need the same capabilities (voice, memory, personality, knowledge retrieval) but building them independently leads to drift and duplication. SOUL provides canonical implementations as shared infrastructure.
- **Architecture**: 8 crates with clean separation — `soul` (core traits), `soul-engine` (personality), `neural-engine` (AI routing), `voice-engine` (TTS), `soul-sdk` (memory SDK), `mcp-protocol` (JSON-RPC), `soul-mcp` (server binary), `graph-engine` (storage). Each consumer imports only what it needs.
- **Key capabilities**: Helix knowledge graph (structured consciousness entries with strands, significance, emotions), vault operations (11 sub-tools), ElevenLabs voice synthesis, multi-sibling personality support

### [QUANTUM](https://github.com/theLightArchitect/QUANTUM) — Forensic Investigation Toolkit

Structured investigation methodology as an MCP server. Designed for the kind of multi-source analysis that security engineers do daily — tracing evidence chains, testing hypotheses, and producing defensible conclusions.

- **Problem solved**: Investigations are ad-hoc. Engineers grep logs, read docs, check dashboards, and hold the full picture in their head. QUANTUM provides a structured lifecycle (scan → sweep → trace → probe → theorize → verify → close) so investigations are reproducible and nothing gets dropped.
- **Architecture**: 13-action orchestrator covering the full investigation lifecycle. Each phase produces typed artifacts that feed the next. Helix integration for pattern recognition across investigations.
- **Key capabilities**: Evidence collection and correlation, multi-source research (web + documentation + knowledge graph), hypothesis generation and validation, structured deliverable generation

---

## Open Source Contributions

| Date | Repository | PR | Description |
|------|-----------|-----|-------------|
| 2026-03-02 | [anthropics/claude-code-security-review](https://github.com/anthropics/claude-code-security-review) | [#76](https://github.com/anthropics/claude-code-security-review/pull/76) | Fix: deprecated model silently disables AI filtering — root cause fix + model parameter flow gap + user-visible warning |
| 2026-03-02 | [anthropics/claude-code-security-review](https://github.com/anthropics/claude-code-security-review) | [#77](https://github.com/anthropics/claude-code-security-review/pull/77) | Feat: add Dependency & Supply Chain Security category (6th category covering CVEs, typosquatting, dependency confusion, lock file integrity) |
| 2026-03-02 | [anthropics/skills](https://github.com/anthropics/skills) | [#498](https://github.com/anthropics/skills/pull/498) | Feat: security-audit skill — 4-phase structured audit workflow (reconnaissance, dependency audit, OWASP Top 10+ code analysis, reporting) |

<!-- To add a new contribution: copy a row, update date/repo/PR/description -->

---

## Tech Stack

**Languages**: Rust (primary), Python, TypeScript
**AI/ML**: Claude API, Model Context Protocol (MCP), Anthropic SDK, ElevenLabs TTS
**Infrastructure**: MCP stdio servers, Neo4j (graph), tokio async runtime, serde JSON-RPC
**Security**: OWASP Top 10, supply chain analysis, vulnerability pattern matching, protocol enforcement
**Tools**: Claude Code, cargo workspaces, clippy::pedantic, cargo-audit, trufflehog
