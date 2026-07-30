"""Extracts scriptural claims from a sermon/podcast transcript and verifies
each one against the grounding agent.
"""

from google.adk.agents import LlmAgent
from google.adk.tools import agent_tool

from .grounding_agent import grounding_agent

sermon_fact_checker_agent = LlmAgent(
    model="gemini-3.5-flash-lite",
    name="sermon_fact_checker_agent",
    description=(
        "Extracts every scriptural claim from a sermon or podcast transcript "
        "and verifies each one against the grounding agent."
    ),
    instruction="""
Given a transcript, extract every distinct scriptural claim or quotation
(reference + what was claimed about it). For each claim, call grounding_agent
to check whether the cited passage says what was claimed. Report each claim
as Accurate / Misquoted / Out-of-context / Reference-not-found, with the
grounded text alongside the flag.
""",
    tools=[agent_tool.AgentTool(agent=grounding_agent)],
)
