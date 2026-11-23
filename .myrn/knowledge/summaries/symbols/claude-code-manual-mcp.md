---
id: claude-code-manual-mcp
type: summary
status: unverified
title: claude_code::manual_mcp
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::manual_mcp
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::manual_mcp@27ccc481fb873e13
map_version: '2'
---
# claude_code::manual_mcp

Formats the shell command a user can run by hand to register the myrn MCP server at local scope.

## Behaviour

- /opt/myrn/bin/myrn -> claude mcp add --scope local myrn -- "/opt/myrn/bin/myrn" mcp
- The path is wrapped in double quotes without escaping
- Pure string formatting with no side effects

## Terms

manual registration, fallback command, register by hand, mcp add

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:57-62`
- **Signature:** `fn manual_mcp(binary: &Path) -> String`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-inspect]]
