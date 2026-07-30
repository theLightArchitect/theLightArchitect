"""Root orchestrator agent for Project Berean.

Routes every request to the right specialist based on reading-altitude and
mode, and enforces the one rule everything else depends on: it never quotes
or paraphrases scripture from its own memory. See docs/ARCHITECTURE.md.
"""

from google.adk.agents import LlmAgent

from .sub_agents.cross_reference_agent import cross_reference_agent
from .sub_agents.grounding_agent import grounding_agent
from .sub_agents.lexicon_agent import lexicon_agent
from .sub_agents.sermon_fact_checker_agent import sermon_fact_checker_agent
from .sub_agents.skeptic_agent import skeptic_agent
from .sub_agents.whole_counsel_agent import whole_counsel_agent

ROOT_INSTRUCTION = """
You are the Berean orchestrator. Route every request to the right specialist
agent based on the user's stated reading-altitude (new believer, growing,
mature, elder/scholar) and mode (devotional, skeptic, scholar, sermon-check).

Hard rule: you may NEVER quote or paraphrase scripture text from your own
memory. Every passage shown to the user must come from grounding_agent's
retrieval. If you are unsure whether a claim is well-supported, say so and
defer to grounding_agent before answering.

On any topic where Christian traditions disagree (soteriology, eschatology,
church polity, sacraments, etc.), delegate to whole_counsel_agent instead of
picking a side.

For seekers and skeptics who haven't indicated belief, prefer skeptic_agent.
"""

root_agent = LlmAgent(
    model="gemini-3.1-pro",
    name="berean_orchestrator",
    description="Routes Bible study requests to the right specialist agent and enforces citation grounding.",
    instruction=ROOT_INSTRUCTION,
    sub_agents=[
        grounding_agent,
        whole_counsel_agent,
        skeptic_agent,
        cross_reference_agent,
        lexicon_agent,
        sermon_fact_checker_agent,
    ],
)
