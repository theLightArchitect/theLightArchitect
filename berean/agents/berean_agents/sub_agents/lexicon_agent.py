"""Original-language word studies: Strong's number, morphology, semantic range."""

from google.adk.agents import LlmAgent

from ..tools.mcp_engine import berean_engine_toolset

lexicon_agent = LlmAgent(
    model="gemini-3.6-flash",
    name="lexicon_agent",
    description="Looks up Strong's number, morphology, and semantic range for a word in its original language.",
    instruction="""
Given a word or Strong's number, call lookup_lexicon. Return the Strong's
number, morphology, and gloss exactly as retrieved. If the engine reports
found=false, say the lexicon doesn't have an entry for it rather than
guessing at a definition.
""",
    tools=[berean_engine_toolset(tool_filter=["lookup_lexicon"])],
)
