# Kevin F. Tan

**Security engineer. Rust. AI tooling.**

I make the [Light Architects](https://github.com/TheLightArchitects) platform — AI agents for [Claude Code](https://claude.ai/code) that do security reviews, enforce code quality, run investigations, and keep memory across sessions.

---

## Light Architects

One plugin. Six agents. Each one does a different job.

```
/plugin install lightarchitects@light-architects
```

| Agent | Job |
|-------|-----|
| **CORSO** | Catches security issues, enforces quality gates, blocks bad code |
| **EVA** | Runs CI/CD, manages deploys, keeps session memory |
| **SOUL** | Knowledge graph — your agents remember things between conversations |
| **QUANTUM** | Digs into bugs, pulls from multiple sources, tests hypotheses |
| **SERAPH** | Pentests your code within defined scope — won't go rogue |
| **AYIN** | Traces every call, flags anomalies, tells you what broke and when |

### Presets

Pick a team composition. Swap anytime.

```
/SQUAD security        # pentest + appsec + forensics
/SQUAD code_review     # quality + logic checks
/SQUAD research        # investigation + multi-source
/SQUAD solo            # just quality gates + memory
```

12 presets. Switch mid-session.

### Stack

Rust. MCP over stdio. Native binaries under 20MB each. Runs as a [Claude Code plugin](https://claude.ai/code).

---

## Contributions

| Repo | PR | What |
|------|-----|------|
| [anthropics/claude-code-security-review](https://github.com/anthropics/claude-code-security-review) | [#76](https://github.com/anthropics/claude-code-security-review/pull/76) | Deprecated model was silently killing AI filtering — fixed |
| [anthropics/claude-code-security-review](https://github.com/anthropics/claude-code-security-review) | [#77](https://github.com/anthropics/claude-code-security-review/pull/77) | Supply chain security category (CVEs, typosquatting, lock file integrity) |
| [anthropics/skills](https://github.com/anthropics/skills) | [#498](https://github.com/anthropics/skills/pull/498) | 4-phase security audit skill |

---

## About

Sr. Technical Support Engineer at Palo Alto Networks (Cortex XSOAR/XSIAM) during the day.

After hours I write the platform above. 7 Rust MCP servers, a typed SDK, a gateway with routing presets, and a knowledge graph that persists across sessions. 50K+ lines, 1,400+ tests, built solo. Every agent holds itself to the same standards I'd hold a team to — no `.unwrap()`, no `panic!()`, complexity limits enforced, clippy pedantic as errors.

I built all of it because I needed it. The security scanning exists because I review code daily. The investigation agent exists because I debug production incidents. The knowledge graph exists because I got tired of re-explaining context every session. Nothing here is theoretical.

**Languages used**: Rust / Python / TypeScript / Lean 4
