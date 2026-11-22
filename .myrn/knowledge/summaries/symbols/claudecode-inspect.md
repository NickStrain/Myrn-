---
id: claudecode-inspect
type: summary
status: unverified
title: ClaudeCode::inspect
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::inspect
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::inspect@eed221504bab23a3
---
# ClaudeCode::inspect

Read-only check that returns Findings. In Hooks mode it checks each event for a hook that exists, points at the current binary, and is installed exactly once. In PullOnly mode it checks for the CLAUDE.md block. It also checks 'claude mcp get myrn' and reports the check as skipped if the CLI is unavailable.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:187-253`
- **Signature:** `fn inspect(&self, ctx: &InstallCtx, mode: Mode) -> Vec<Finding>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[hooks-json-find]], [[claude-code-settings-path]], [[claude-code-instructions-path]], [[claude-code-manual-mcp]], [[claude-code-claude-cli]], [[pull-only-has-block]]
