from berean_agents.agent import root_agent

EXPECTED_SUB_AGENT_NAMES = {
    "grounding_agent",
    "whole_counsel_agent",
    "skeptic_agent",
    "cross_reference_agent",
    "lexicon_agent",
    "sermon_fact_checker_agent",
}


def test_root_agent_has_all_sub_agents():
    names = {agent.name for agent in root_agent.sub_agents}
    assert names == EXPECTED_SUB_AGENT_NAMES


def test_root_agent_has_no_direct_tools():
    # The orchestrator must never answer directly with scripture text or
    # data — it only ever delegates to a specialist sub-agent.
    assert not root_agent.tools
