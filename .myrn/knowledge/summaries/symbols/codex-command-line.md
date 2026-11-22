---
id: codex-command-line
type: summary
status: unverified
title: codex::command_line
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::command_line
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::command_line@ca8d92159fb6c225
---
# codex::command_line

Builds the hook command as the binary path in double quotes followed by 'hook codex <event>'. The quoting lets the command run under POSIX shells and cmd.exe.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:46-48`
- **Signature:** `fn command_line(binary: &Path, event: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-install]], [[codex-inspect]]
