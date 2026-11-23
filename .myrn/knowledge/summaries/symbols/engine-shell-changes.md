---
id: engine-shell-changes
type: summary
status: unverified
title: Engine::shell_changes
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::shell_changes
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::shell_changes@0daeb86a551f8066
map_version: '2'
---
# Engine::shell_changes

After a shell command, refreshes the index and returns the changed files and the symbols whose body changed, appeared or disappeared.

## Behaviour

- Compares against a snapshot taken before the refresh; returns empty lists when nothing changed
- Symbols that vanished from a changed file are included
- Symbol ids are sorted and de-duplicated

## Terms

shell edits, sed, formatter, code generator, changed files, shell command effects

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:958-996`
- **Signature:** `fn shell_changes(&mut self) -> Result<(Vec<String>, Vec<String>)>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-refresh]], [[graph-node-graph]]
- **Called by:** [[engine-capture]]
