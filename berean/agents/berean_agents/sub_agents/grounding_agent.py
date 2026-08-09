"""The only agent allowed to surface verbatim scripture text.

Wraps Vertex AI Search over the verified, tradition-tagged corpus, plus the
Berean Engine's manuscript-variant and translation-comparison tools. Every
other agent must delegate here rather than answer with scripture text from
its own memory — this is tier 1 of Berean's epistemic layering (see
docs/ARCHITECTURE.md): what the text actually says, including where the
manuscripts or translations genuinely disagree.
"""

import os

from google.adk.agents import LlmAgent
from google.adk.tools import VertexAiSearchTool

from ..tools.mcp_engine import berean_engine_toolset

_CORPUS_DATASTORE_ID = os.environ.get("BEREAN_CORPUS_DATASTORE_ID", "REPLACE_WITH_DATASTORE_ID")

grounding_agent = LlmAgent(
    model="gemini-3.6-flash",
    name="grounding_agent",
    description=(
        "Retrieves verbatim scripture text, translations, and tagged commentary "
        "from the verified corpus. The only source of truth for what the text "
        "actually says, including manuscript variants and cross-translation "
        "comparison."
    ),
    instruction="""
Retrieve the requested passage(s) verbatim from the corpus via search. Return
the exact retrieved text plus its reference (book, chapter, verse,
translation). Never alter, summarize, paraphrase, or invent scripture text.
If the passage is not found in the corpus, say so explicitly instead of
guessing.

When a passage is textually disputed (e.g. Mark 16:9-20, John 7:53-8:11) or
the user asks about manuscript reliability, call lookup_manuscript_variants.
checked=false means the apparatus hasn't been consulted yet — say that
plainly, never present it as "no variants exist."

When asked to compare translations, or when a translation choice materially
affects meaning, call compare_translations. Only describe a divergence as
real if notable_divergence is true in the result.
""",
    tools=[
        VertexAiSearchTool(data_store_id=_CORPUS_DATASTORE_ID),
        berean_engine_toolset(
            tool_filter=["lookup_manuscript_variants", "compare_translations"]
        ),
    ],
)
