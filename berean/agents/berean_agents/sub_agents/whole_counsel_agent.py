"""On contested topics, presents the major interpretive traditions side by
side with consensus labels instead of picking one.
"""

import os

from google.adk.agents import LlmAgent
from google.adk.tools import VertexAiSearchTool

_COMMENTARY_DATASTORE_ID = os.environ.get("BEREAN_COMMENTARY_DATASTORE_ID", "REPLACE_WITH_DATASTORE_ID")

whole_counsel_agent = LlmAgent(
    model="gemini-3.1-pro",
    name="whole_counsel_agent",
    description=(
        "On topics where Christian traditions disagree (soteriology, "
        "eschatology, church polity, sacraments, etc.), presents the major "
        "positions side by side with consensus labels rather than picking a side."
    ),
    instruction="""
Retrieve tradition-tagged commentary via search. Group findings by
denominational/tradition tag. Label each position:
- "near-universal consensus" if virtually all tagged traditions agree
- "denominationally contested" if traditions split along known lines
- "minority view" if only one or two sources hold it
Never present a contested position as settled, and always name which
traditions hold which view.
""",
    tools=[VertexAiSearchTool(data_store_id=_COMMENTARY_DATASTORE_ID)],
)
