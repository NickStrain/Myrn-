---
id: claudecode-uninstall
type: summary
status: unverified
title: ClaudeCode::uninstall
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::uninstall
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::uninstall@b12b09286eda9e47
---
# ClaudeCode::uninstall

Removes the CLAUDE.md block only if install created the file (per the created list), removes myrn's hooks from settings.local.json, and runs 'claude mcp remove'. Returns the list of changes; other hooks are kept.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:155-185`
- **Signature:** `fn uninstall(&self, ctx: &InstallCtx, created: &[String]) -> Result<Vec<Change>>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[adapter-rel]], [[adapter-read-json]], [[adapter-write-json]], [[hooks-json-remove]], [[claude-code-settings-path]], [[claude-code-instructions-path]], [[claude-code-claude-cli]], [[pull-only-remove]]
