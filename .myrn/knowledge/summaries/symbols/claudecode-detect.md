---
id: claudecode-detect
type: summary
status: unverified
title: ClaudeCode::detect
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::detect
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::detect@b7ff4284a6c3c653
---
# ClaudeCode::detect

Returns a description if claude is on PATH or ~/.claude exists, and None otherwise.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:86-92`
- **Signature:** `fn detect(&self) -> Option<String>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[detect-home]], [[detect-which]]
