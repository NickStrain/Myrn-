---
id: claude-code-handler
type: summary
status: unverified
title: claude_code::handler
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::handler
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::handler@6621d768eeb1c040
---
# claude_code::handler

Builds a command-type hook object with command set to the binary path, args ["hook", "claude", event], and the given timeout. No shell is involved.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:48-55`
- **Signature:** `fn handler(binary: &Path, event: &str, timeout: u64) -> Value`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Called by:** [[claudecode-install]]
