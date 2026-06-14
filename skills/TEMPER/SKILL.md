---
name: TEMPER
description: "Shared code-quality protocol invoked by all domain agents to take code, analyze it, and make it professional in a single pass. Hybrid architecture: deterministic gates first (format, lint, type, test, security — cheap, 100% coverage), then rubric-driven LLM review for the subjective qualities deterministic tools cannot judge (clarity, naming, design, simplicity), then a fix-and-reverify loop, then a human gate at any irreversible step. Output: TEMPER Verdict (GREEN/YELLOW/RED/HALT) + structured findings (JSON) + evidence grades + gaps + IEEE citations. Domain-agnostic via {DOMAIN} parameter."
user-invocable: false
context: fork
version: 1.0.0
---

# TEMPER — Shared Code-Quality Protocol

> Working is the floor. Professional is the bar. Every finding cites its rule; every fix is re-verified.

Shared protocol invoked by **all seven domain agents**. Single source of truth for turning code into professional code. "One shot" means **one command** — not one model call. Confidence is earned through layers, not a single hopeful pass.

The protocol parameterizes on `{DOMAIN}` — the invoking agent fills it with its own domain (table at bottom). Tools and rubric weights shift by domain; the gate sequence does not.

-----

## Layer Architecture

Four layers, in order. A layer's output feeds the next. **Layer 1 is deterministic and blocking. Layer 2 is inferential. Never let Layer 2 ship without re-running Layer 1.**

### Layer 1 — Deterministic Gates (blocking, 100% coverage, ~ms)

Objective checks: same input, same output, every time. Run on the full diff. A hard failure here aborts the run — the correctness floor is not met.

|Gate    |Tools (by language)                           |Hard-fail on         |
|--------|----------------------------------------------|---------------------|
|Format  |`prettier` / `ruff format` / `gofmt`          |— (auto-fix)         |
|Lint    |`eslint` / `ruff` / `pylint` / `golangci-lint`|error-level rules    |
|Type    |`tsc` / `mypy` / `pyright`                    |type errors          |
|Build   |`tsc` / `cargo build` / `gradle`              |compile failure      |
|Test    |`jest` / `vitest` / `pytest` / `go test`      |any test failure     |
|Security|`semgrep` / `bandit` / `codeql`               |high/critical finding|


> Reach for code-based graders first for anything checkable in code. They cost near-zero and run on 100% of outputs.

Graceful skip: tool absent (`which <tool>` fails) → note the gap as a finding, continue. A skipped gate lowers verdict confidence.

### Layer 2 — Rubric Review (inferential, judgment only)

Call the LLM **only for what Layer 1 cannot judge** — the subjective qualities. Give it real context: full file contents, related/changed files, and the domain conventions file. Force structured JSON output (schema below).

Add this inferential check only where it reduces meaningful risk. It is calibrated, not trusted: validate against human-labeled examples before using as a gate.

### Layer 3 — Fix & Re-Verify (loop)

If findings are `auto_fixable`, apply them, then **re-run Layer 1 in full**. An unverified fix can silently break what was passing. Loop until no auto-fixable findings remain or a fix iteration cap (default 3) is hit.

### Layer 4 — Human Gate

Emit the verdict + findings + diff. A human approves anything irreversible (merge, deploy, schema/data migration, deletion). The protocol never self-merges.

-----

## The Rubric — what "professional" means

Encode the three tiers. Each criterion carries a severity and a domain-weight.

|Tier            |Criterion     |Checks                                                   |Severity|
|----------------|--------------|---------------------------------------------------------|:------:|
|**Correct**     |Functionality |Meets spec, edge cases handled, fails safely             |BLOCKER |
|**Correct**     |Tested        |Behavior covered, regressions guarded                    |BLOCKER |
|**Good**        |Clarity       |A new reader understands intent quickly                  |MAJOR   |
|**Good**        |Naming        |Names reveal intent; no decoding required                |MAJOR   |
|**Good**        |Simplicity    |Simplest solution that fits; complexity only where needed|MAJOR   |
|**Good**        |Consistency   |Matches codebase style and conventions                   |MINOR   |
|**Good**        |DRY           |No risky duplicated logic                                |MINOR   |
|**Professional**|Error handling|Graceful, meaningful, no silent failures                 |MAJOR   |
|**Professional**|Security      |Input validated; no injection/secrets/unsafe deps        |BLOCKER |
|**Professional**|Containment   |No sprawl, no needless dependencies, modular             |MINOR   |
|**Professional**|Comments      |Explain *why*, not *what*; code self-explanatory         |MINOR   |

-----

## Dispatch Protocol

```
Step 1 — Layer 1 (sequential, blocking):
  format → lint → type → build → test → security
  any BLOCKER hard-fail → abort, emit RED/HALT verdict

Step 2 — Layer 2 (single LLM call):
  llm_review(
    code        = diff,
    context     = related_files + {DOMAIN}_conventions,
    rubric      = RUBRIC,
    output      = FINDING_SCHEMA   # JSON, validated on return
  )
  malformed output → reject, retry once, else flag UNVERIFIED

Step 3 — Layer 3 (loop, cap 3):
  apply auto_fixable findings → GOTO Step 1

Step 4 — Layer 4:
  emit verdict + findings + diff → human approves irreversible steps
```

-----

## Finding Schema (structured output — non-negotiable)

```json
{
  "findings": [
    {
      "id": "string",
      "tier": "correct | good | professional",
      "criterion": "string",
      "severity": "blocker | major | minor",
      "file": "path",
      "line": 0,
      "rule": "rubric-criterion-or-tool-rule-id",
      "why": "one sentence, no hedging",
      "suggested_fix": "string",
      "auto_fixable": true,
      "evidence_grade": "DETERMINISTIC | INFERENTIAL"
    }
  ],
  "gates_run": ["format","lint","type","build","test","security"],
  "gates_skipped": [{"gate":"string","reason":"string"}],
  "verdict": "GREEN | YELLOW | RED | HALT",
  "confidence": 0.00
}
```

`evidence_grade`: **DETERMINISTIC** (Layer 1 tool) outranks **INFERENTIAL** (Layer 2 judge). An INFERENTIAL finding unconfirmed by any deterministic signal is UNVERIFIED until a human or a test corroborates it.

-----

## Verdict Bands

|Verdict   |Meaning                                               |Action                      |
|----------|------------------------------------------------------|----------------------------|
|**GREEN** |No BLOCKER/MAJOR; Layer 1 clean                       |Ready for human review/merge|
|**YELLOW**|MINOR findings only, or a gate skipped                |Mergeable with noted debt   |
|**RED**   |One or more MAJOR, or auto-fix loop exhausted         |Return to author/agent      |
|**HALT**  |BLOCKER: failing test, type error, or security finding|Stop. No merge. No deploy.  |

-----

## Covenant (inherited from Canon V, §2)

- **No hedging.** Forbidden in findings: "likely", "probably", "seems to", "should work", "I think". State the rule and the evidence, or omit the finding.
- **Numeric confidence** on every verdict (0.00–1.00).
- **Gaps are findings.** A skipped gate or absent tool is named explicitly, never buried.
- **Every claim cites its rule.** Tool rule-id or rubric criterion. No anonymous opinions.

-----

## Domain Parameterization

Each agent fills `{DOMAIN}` with its own row. Gate set and rubric weights shift; the four-layer sequence is invariant.

|`{DOMAIN}`|Conventions file   |Heaviest rubric weight|Domain-specific Layer-1 add-ons                              |
|----------|-------------------|----------------------|-------------------------------------------------------------|
|CORSO     |`…/CORSO/AGENTS.md`  |Security            |`cargo audit` + `cargo deny`; `clippy::pedantic` as errors   |
|EVA       |`…/EVA/AGENTS.md`    |Tested              |pipeline lint (`actionlint`); deploy dry-run before promotion|
|SOUL      |`…/SOUL/AGENTS.md`   |Consistency         |graph-schema + migration integrity check                     |
|QUANTUM   |`…/QUANTUM/AGENTS.md`|Functionality       |repro harness; property tests (`proptest`)                   |
|SERAPH    |`…/SERAPH/AGENTS.md` |Security            |fuzz (`cargo fuzz`); scope-boundary assertion                |
|AYIN      |`…/AYIN/AGENTS.md`   |Error handling      |trace/semconv lint (`lasdlc.<domain>.<action>` keys)         |
|LÆX       |`…/LAEX/AGENTS.md`   |Functionality       |proof build (`lake build`); spec-compliance check            |

Graceful skip: no conventions file for a domain → fall back to the universal rubric, note reduced precision.

-----

## Bibliography (IEEE)

[1] Alibaba, "open-code-review: hybrid deterministic pipelines + LLM agent," GitHub. <https://github.com/alibaba/open-code-review>

[2] MindStudio, "Structured AI Coding Workflow with Deterministic and Agentic Nodes." <https://www.mindstudio.ai/blog/structured-ai-coding-workflow-deterministic-agentic-nodes>

[3] DigitalApplied, "Building an AI Agent Evaluation Pipeline: 2026 Methodology." <https://www.digitalapplied.com/blog/ai-agent-evaluation-pipeline-2026-testing-methodology>

[4] ContextQA, "LLM Testing Tools and Frameworks in 2026." <https://contextqa.com/blog/llm-testing-tools-frameworks-2026/>

[5] IBM, "What Is Code Quality?" <https://www.ibm.com/think/topics/code-quality>

[6] ai-boost, "awesome-harness-engineering," GitHub. <https://github.com/ai-boost/awesome-harness-engineering>
