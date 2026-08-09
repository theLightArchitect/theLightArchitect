"""Wiring smoke tests: agent tree structure and tool scoping only. These
inspect McpToolset's tool_filter without connecting to the Berean Engine, so
they run without BEREAN_ENGINE_BIN set or the binary built. For a real
end-to-end check against the running engine, call
`await toolset.get_tools()` with BEREAN_ENGINE_BIN pointed at a built binary.
"""

from google.adk.tools.mcp_tool import McpToolset

from berean_agents.agent import root_agent

EXPECTED_SUB_AGENT_NAMES = {
    "grounding_agent",
    "whole_counsel_agent",
    "skeptic_agent",
    "cross_reference_agent",
    "lexicon_agent",
    "sermon_fact_checker_agent",
}

# The one deliberate exception to "root never touches content tools directly":
# pastoral triage and journal continuity are cross-cutting root concerns, not
# specialist ones. Everything else must stay behind a sub-agent.
ROOT_SAFETY_TOOLS = {"detect_pastoral_signal", "read_journal", "write_journal"}


def _mcp_tool_filter(agent):
    mcp_tools = [t for t in agent.tools if isinstance(t, McpToolset)]
    assert len(mcp_tools) == 1, f"{agent.name} should have exactly one McpToolset"
    return set(mcp_tools[0].tool_filter)


def test_root_agent_has_all_sub_agents():
    names = {agent.name for agent in root_agent.sub_agents}
    assert names == EXPECTED_SUB_AGENT_NAMES


def test_root_agent_only_has_safety_and_continuity_tools():
    # The orchestrator must never answer directly with scripture text or
    # content-bearing data (manuscript variants, confessions, patristics,
    # cross-refs, lexicon) — only the cross-cutting pastoral/journal tools.
    assert _mcp_tool_filter(root_agent) == ROOT_SAFETY_TOOLS


def test_grounding_agent_scoped_to_verbatim_fidelity_tools():
    from berean_agents.sub_agents.grounding_agent import grounding_agent

    assert _mcp_tool_filter(grounding_agent) == {
        "lookup_manuscript_variants",
        "compare_translations",
    }


def test_whole_counsel_agent_scoped_to_primary_source_tools():
    from berean_agents.sub_agents.whole_counsel_agent import whole_counsel_agent

    assert _mcp_tool_filter(whole_counsel_agent) == {
        "lookup_confession",
        "search_patristics",
    }


def test_no_two_agents_share_the_same_engine_tool():
    # Each Berean Engine tool should belong to exactly one agent's contract —
    # overlap would blur which agent is accountable for a given claim.
    from berean_agents.sub_agents.cross_reference_agent import cross_reference_agent
    from berean_agents.sub_agents.grounding_agent import grounding_agent
    from berean_agents.sub_agents.lexicon_agent import lexicon_agent
    from berean_agents.sub_agents.whole_counsel_agent import whole_counsel_agent

    filters = [
        _mcp_tool_filter(root_agent),
        _mcp_tool_filter(grounding_agent),
        _mcp_tool_filter(whole_counsel_agent),
        _mcp_tool_filter(cross_reference_agent),
        _mcp_tool_filter(lexicon_agent),
    ]
    all_tools = [tool for filter_set in filters for tool in filter_set]
    assert len(all_tools) == len(set(all_tools))
