---
id: claudecode-install
type: summary
status: unverified
title: ClaudeCode::install
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::install
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::install@67b730181bbba2cd
map_version: '2'
---
# ClaudeCode::install

Writes hooks or the pull-only instruction block into the repo, removing old myrn hooks first. Then runs the MCP registration through the claude CLI, and adds a manual command to the result if that fails.

## Behaviour

- Hooks mode -> replaces myrn hooks in settings.local.json, keeping other hooks and permissions
- PullOnly mode -> strips myrn hooks if present, then adds the CLAUDE.md block
- MCP step always runs remove then add; a failed add or missing CLI adds a manual line, not an error

## Terms

setup, hook install, mcp registration, pull only mode, idempotent, reinstall

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:94-153`
- **Signature:** `fn install(&self, ctx: &InstallCtx, mode: Mode) -> Result<Installed>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[adapter-write-json]], [[hooks-json-remove]], [[hooks-json-add]], [[claude-code-settings-path]], [[claude-code-instructions-path]], [[claude-code-handler]], [[claude-code-manual-mcp]], [[claude-code-claude-cli]], [[pull-only-add]]
