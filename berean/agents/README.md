# Berean Agents

ADK multi-agent system for Project Berean. See `../docs/ARCHITECTURE.md` for
the design rationale.

## Setup

```bash
cd berean/agents
python -m venv .venv && source .venv/bin/activate
pip install -e ".[dev]"
```

Set the datastore IDs the grounding/whole-counsel/skeptic agents depend on
(create these in Vertex AI Search first — they're not provisioned by this
scaffold):

```bash
export BEREAN_CORPUS_DATASTORE_ID=...
export BEREAN_COMMENTARY_DATASTORE_ID=...
export BEREAN_SKEPTIC_DATASTORE_ID=...
```

If you've built the Berean Engine (`../engine`), point at the binary:

```bash
export BEREAN_ENGINE_BIN=../engine/target/release/berean-engine
```

## Run

```bash
adk web        # local dev UI
adk run .      # CLI
```

## Test

```bash
pytest
```

Only `test_agent_wiring.py` exists so far — a smoke test that the agent tree
builds correctly. It doesn't call any model or datastore.
