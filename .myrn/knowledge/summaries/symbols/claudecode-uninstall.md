---
id: claudecode-uninstall
type: summary
status: unverified
title: ClaudeCode::uninstall
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::uninstall
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::uninstall@b12b09286eda9e47
map_version: '2'
---
# ClaudeCode::uninstall

Removes the CLAUDE.md instruction block, strips myrn hooks from settings.local.json, and unregisters the MCP server. Returns the list of changes made.

## Behaviour

- The created list is passed to pull_only::remove to decide about the file
- Missing settings file is skipped; the file is only written if a hook was removed
- MCP change is recorded only when the CLI runs and exits with success

## Terms

teardown, cleanup, remove hooks, undo install

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:155-185`
- **Signature:** `fn uninstall(&self, ctx: &InstallCtx, created: &[String]) -> Result<Vec<Change>>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[adapter-write-json]], [[hooks-json-remove]], [[claude-code-settings-path]], [[claude-code-instructions-path]], [[claude-code-claude-cli]], [[pull-only-remove]]
