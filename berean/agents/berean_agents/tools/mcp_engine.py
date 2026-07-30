"""Shared connection to the Berean Engine (Rust MCP server).

The engine owns the cross-reference graph and lexicon data, enforcing the
same verbatim-citation contract as the grounding agent's Vertex AI Search
tool, just for structured lookups instead of full-text search. See
engine/src/corpus.rs for the contract: return the data with its source, or
say "not found" — never guess.
"""

import os

from google.adk.tools.mcp_tool import McpToolset
from google.adk.tools.mcp_tool.mcp_session_manager import StdioConnectionParams
from mcp import StdioServerParameters

_ENGINE_BIN = os.environ.get(
    "BEREAN_ENGINE_BIN",
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "engine", "target", "release", "berean-engine"),
)


def berean_engine_toolset(tool_filter: list[str] | None = None) -> McpToolset:
    """Build an McpToolset connected to the Berean Engine over stdio.

    tool_filter restricts which engine tools a given agent can call (e.g. the
    lexicon agent should only ever see lookup_lexicon).
    """
    return McpToolset(
        connection_params=StdioConnectionParams(
            server_params=StdioServerParameters(command=_ENGINE_BIN, args=[]),
            timeout=10,
        ),
        tool_filter=tool_filter,
    )
