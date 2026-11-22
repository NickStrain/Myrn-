---
id: claude-code-is-ours
type: summary
status: unverified
title: claude_code::is_ours
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::is_ours
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::is_ours@7463cce1272e726d
---
# claude_code::is_ours

Returns true when a hook's args begin with ["hook", "claude"]. Install and uninstall use this to replace only myrn's own hooks and leave others untouched.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:39-46`
- **Signature:** `fn is_ours(h: &Value) -> bool`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
