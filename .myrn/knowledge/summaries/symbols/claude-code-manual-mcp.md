---
id: claude-code-manual-mcp
type: summary
status: unverified
title: claude_code::manual_mcp
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::manual_mcp
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::manual_mcp@27ccc481fb873e13
---
# claude_code::manual_mcp

Returns the shell line 'claude mcp add --scope local myrn -- "<binary>" mcp' for the user to run when automatic registration is not possible.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:57-62`
- **Signature:** `fn manual_mcp(binary: &Path) -> String`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-inspect]]
