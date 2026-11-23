---
id: codex-command-line
type: summary
status: unverified
title: codex::command_line
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::command_line
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::command_line@ca8d92159fb6c225
map_version: '2'
---
# codex::command_line

Builds the hook command string: the quoted binary path, then 'hook codex', then the event name.

## Behaviour

- /opt/myrn and SessionStart -> quoted path then 'hook codex SessionStart'
- a path with spaces stays inside the quotes
- output contains the ' hook codex ' marker that is_ours checks for

## Terms

hook command, quoted command, shell command text

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:46-48`
- **Signature:** `fn command_line(binary: &Path, event: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Called by:** [[codex-install]], [[codex-inspect]]
