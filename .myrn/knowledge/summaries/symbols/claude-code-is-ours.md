---
id: claude-code-is-ours
type: summary
status: unverified
title: claude_code::is_ours
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::is_ours
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::is_ours@7463cce1272e726d
map_version: '2'
---
# claude_code::is_ours

Reports whether a hook entry was written by myrn, by checking that its args begin with hook and claude.

## Behaviour

- args [hook, claude, SessionStart] -> true
- args [hook, codex] or no args field -> false
- args with fewer than two strings -> false

## Terms

ownership check, owned entry, myrn hook, identify hook

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:39-46`
- **Signature:** `fn is_ours(h: &Value) -> bool`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
