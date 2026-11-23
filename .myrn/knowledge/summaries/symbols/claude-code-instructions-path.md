---
id: claude-code-instructions-path
type: summary
status: unverified
title: claude_code::instructions_path
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::instructions_path
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::instructions_path@dc7fa900736faaca
map_version: '2'
---
# claude_code::instructions_path

Returns CLAUDE.md at the repo root, where the pull-only instruction block is written.

## Behaviour

- /repo -> /repo/CLAUDE.md
- Used by install, uninstall and inspect in pull-only mode

## Terms

instruction file, project instructions, memory file

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:35-37`
- **Signature:** `fn instructions_path(root: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[claudecode-inspect]]
