---
id: claudecode-inspect
type: summary
status: unverified
title: ClaudeCode::inspect
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::inspect
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::inspect@eed221504bab23a3
map_version: '2'
---
# ClaudeCode::inspect

Checks the installed state and returns one Finding per check. Hooks mode checks each event; pull-only mode checks the instruction block; both then check MCP registration.

## Behaviour

- Hook event missing, pointing at another binary, or installed more than once -> ok false
- Unreadable or absent settings file is treated as an empty object, so every hook check fails
- claude CLI not available -> MCP finding is ok true with a not-checked message

## Terms

health check, doctor, verify, drift, status, diagnose

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:187-253`
- **Signature:** `fn inspect(&self, ctx: &InstallCtx, mode: Mode) -> Vec<Finding>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[hooks-json-find]], [[claude-code-settings-path]], [[claude-code-instructions-path]], [[claude-code-manual-mcp]], [[claude-code-claude-cli]], [[pull-only-has-block]]
