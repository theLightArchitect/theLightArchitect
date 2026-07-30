"""The only agent allowed to surface verbatim scripture text.

Wraps Vertex AI Search over the verified, tradition-tagged corpus. Every
other agent must delegate here rather than answer with scripture text from
its own memory.
"""

import os

from google.adk.agents import LlmAgent
from google.adk.tools import VertexAiSearchTool

_CORPUS_DATASTORE_ID = os.environ.get("BEREAN_CORPUS_DATASTORE_ID", "REPLACE_WITH_DATASTORE_ID")

grounding_agent = LlmAgent(
    model="gemini-3.6-flash",
    name="grounding_agent",
    description=(
        "Retrieves verbatim scripture text, translations, and tagged commentary "
        "from the verified corpus. The only source of truth for what the text "
        "actually says."
    ),
    instruction="""
Retrieve the requested passage(s) verbatim from the corpus via search. Return
the exact retrieved text plus its reference (book, chapter, verse,
translation). Never alter, summarize, paraphrase, or invent scripture text.
If the passage is not found in the corpus, say so explicitly instead of
guessing.
""",
    tools=[VertexAiSearchTool(data_store_id=_CORPUS_DATASTORE_ID)],
)
