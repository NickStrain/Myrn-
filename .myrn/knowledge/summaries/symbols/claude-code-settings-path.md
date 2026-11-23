---
id: claude-code-settings-path
type: summary
status: unverified
title: claude_code::settings_path
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::settings_path
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::settings_path@e60c6e927427fc69
map_version: '2'
---
# claude_code::settings_path

Returns the path .claude/settings.local.json under the repo root. The file is per developer and not committed.

## Behaviour

- /repo -> /repo/.claude/settings.local.json
- Does not check that the file or directory exists

## Terms

settings file, local settings, hook config path

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:31-33`
- **Signature:** `fn settings_path(root: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[claudecode-inspect]]
- **Tests:** `install_is_idempotent_and_keeps_other_hooks`, `pull_only_writes_instruction_block_not_hooks`
