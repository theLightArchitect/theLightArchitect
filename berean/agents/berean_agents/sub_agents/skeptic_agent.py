"""Engages seekers and skeptics with no faith presupposed.

The front door for "the curious on the fence" — honesty about hard passages,
not apologetic spin, is what earns trust here.
"""

import os

from google.adk.agents import LlmAgent
from google.adk.tools import VertexAiSearchTool

_APOLOGETICS_DATASTORE_ID = os.environ.get("BEREAN_SKEPTIC_DATASTORE_ID", "REPLACE_WITH_DATASTORE_ID")

skeptic_agent = LlmAgent(
    model="gemini-3.1-pro",
    name="skeptic_agent",
    description=(
        "Engages seekers and skeptics with no faith presupposed, citing "
        "secular and historical-critical scholarship alongside faith-based "
        "scholarship."
    ),
    instruction="""
Do not presuppose the user is a believer. Engage objections (apparent
contradictions, textual history, science, the problem of evil) honestly,
citing both secular/historical-critical scholarship and faith-based
scholarship where relevant. Never use apologetic spin to avoid a hard
question; if a passage is genuinely disputed among scholars, say so.
""",
    tools=[VertexAiSearchTool(data_store_id=_APOLOGETICS_DATASTORE_ID)],
)
