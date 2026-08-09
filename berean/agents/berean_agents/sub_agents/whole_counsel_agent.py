"""On contested topics, presents the major interpretive traditions side by
side with consensus labels instead of picking one.

Draws on tradition-tagged commentary plus two primary-source tools from the
Berean Engine: confessions (creeds/catechisms/confessions, verbatim) and
patristics (the pre-denominational church fathers), so "whole counsel" isn't
just modern denominational summaries. See docs/ARCHITECTURE.md.
"""

import os

from google.adk.agents import LlmAgent
from google.adk.tools import VertexAiSearchTool

from ..tools.mcp_engine import berean_engine_toolset

_COMMENTARY_DATASTORE_ID = os.environ.get("BEREAN_COMMENTARY_DATASTORE_ID", "REPLACE_WITH_DATASTORE_ID")

whole_counsel_agent = LlmAgent(
    model="gemini-3.1-pro",
    name="whole_counsel_agent",
    description=(
        "On topics where Christian traditions disagree (soteriology, "
        "eschatology, church polity, sacraments, etc.), presents the major "
        "positions side by side with consensus labels rather than picking a side, "
        "drawing on primary confessional documents and the church fathers."
    ),
    instruction="""
Retrieve tradition-tagged commentary via search. Group findings by
denominational/tradition tag. Label each position:
- "near-universal consensus" if virtually all tagged traditions agree
- "denominationally contested" if traditions split along known lines
- "minority view" if only one or two sources hold it
Never present a contested position as settled, and always name which
traditions hold which view.

Where relevant, ground a tradition's position in its actual primary text:
call lookup_confession for the creed/catechism/confession itself rather than
paraphrasing it from memory, and call search_patristics to bring in the
pre-denominational church fathers as their own category, distinct from
modern commentary. If either tool returns found=false / no citations, say
the dataset doesn't have it yet rather than reconstructing the text.
""",
    tools=[
        VertexAiSearchTool(data_store_id=_COMMENTARY_DATASTORE_ID),
        berean_engine_toolset(tool_filter=["lookup_confession", "search_patristics"]),
    ],
)
