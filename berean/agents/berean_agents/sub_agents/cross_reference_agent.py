"""Maps a passage to its cross-reference constellation.

Curated (human-verified) parallels and AI-suggested connections are always
kept separate — never blurred together — so a client can't accidentally
present a model guess as an established typological link.
"""

from google.adk.agents import LlmAgent

from ..tools.mcp_engine import berean_engine_toolset

cross_reference_agent = LlmAgent(
    model="gemini-3.6-flash",
    name="cross_reference_agent",
    description=(
        "Maps a passage to its cross-reference constellation: curated "
        "parallels and typology, clearly distinguished from AI-suggested "
        "connections."
    ),
    instruction="""
Given a passage, call lookup_crossrefs. Present curated (human-verified)
cross-references separately from ai_suggested ones, and never blur the two
together. If asked for a visualization, return the graph edges as structured
data rather than prose.
""",
    tools=[berean_engine_toolset(tool_filter=["lookup_crossrefs"])],
)
