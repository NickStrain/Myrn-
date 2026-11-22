---
id: claude-code-instructions-path
type: summary
status: unverified
title: claude_code::instructions_path
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::instructions_path
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::instructions_path@dc7fa900736faaca
---
# claude_code::instructions_path

Returns root/CLAUDE.md, the file that receives the pull-only instruction block.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:35-37`
- **Signature:** `fn instructions_path(root: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[claudecode-inspect]]
