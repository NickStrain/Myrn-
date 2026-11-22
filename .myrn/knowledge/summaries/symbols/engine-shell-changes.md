---
id: engine-shell-changes
type: summary
status: unverified
title: Engine::shell_changes
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::shell_changes
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::shell_changes@0daeb86a551f8066
---
# Engine::shell_changes

Refreshes the index after a shell command and returns the changed paths and the symbols whose body hash changed, appeared or disappeared. Both lists are empty when nothing changed.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:892-930`
- **Signature:** `fn shell_changes(&mut self) -> Result<(Vec<String>, Vec<String>)>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-refresh]], [[graph-node-graph]]
- **Called by:** [[engine-capture]]
