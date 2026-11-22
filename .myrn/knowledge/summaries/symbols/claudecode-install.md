---
id: claudecode-install
type: summary
status: unverified
title: ClaudeCode::install
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::install
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::install@67b730181bbba2cd
---
# ClaudeCode::install

In Hooks mode, replaces myrn's hooks for all six EVENTS and keeps other hooks. In PullOnly mode, removes myrn's hooks and writes the CLAUDE.md block. It then runs 'claude mcp remove' and 'claude mcp add --scope local' and adds the manual command to Installed.manual if the CLI is missing or fails. It writes files, so it is idempotent but not read-only.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:94-153`
- **Signature:** `fn install(&self, ctx: &InstallCtx, mode: Mode) -> Result<Installed>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[adapter-write-json]], [[hooks-json-remove]], [[hooks-json-add]], [[claude-code-settings-path]], [[claude-code-instructions-path]], [[claude-code-handler]], [[claude-code-manual-mcp]], [[claude-code-claude-cli]], [[pull-only-add]]
