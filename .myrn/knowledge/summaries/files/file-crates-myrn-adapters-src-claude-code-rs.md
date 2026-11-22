---
id: file-crates-myrn-adapters-src-claude-code-rs
type: summary
status: unverified
title: crates/myrn-adapters/src/claude_code.rs
source: myrn summarize
about: file:crates/myrn-adapters/src/claude_code.rs
anchors:
- file:crates/myrn-adapters/src/claude_code.rs@a17ac9b0461f21ea
---
# crates/myrn-adapters/src/claude_code.rs

Claude Code adapter. It installs and checks myrn's hooks in .claude/settings.local.json, the pull-only instruction block in CLAUDE.md, and the MCP registration via the claude CLI. It also translates Claude Code hook payloads into myrn events and formats hook output back to Claude Code.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs`
- **In:** [[dir-crates-myrn-adapters-src]]
- **Defines:** [[claude-code-claudecode]], [[claude-code-settings-path]], [[claude-code-instructions-path]], [[claude-code-is-ours]], [[claude-code-handler]], [[claude-code-manual-mcp]], [[claude-code-claude-cli]], [[claudecode-name]], [[claudecode-display-name]], [[claudecode-detect]], [[claudecode-install]], [[claudecode-uninstall]], [[claudecode-inspect]], [[claudecode-translate]], [[claudecode-format]], [[tests-ctx]], [[tests-payload]]
- **Imports:** [[file-crates-myrn-adapters-src-adapter-rs]], [[file-crates-myrn-adapters-src-detect-rs]], [[file-crates-myrn-adapters-src-lib-rs]], [[file-crates-myrn-adapters-src-pull-only-rs]], [[file-crates-myrn-model-src-event-rs]]
- **Imported by:** [[file-crates-myrn-adapters-src-lib-rs]]
