---
id: api-asks-about-history
type: summary
status: unverified
title: api::asks_about_history
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::asks_about_history
anchors:
- sym:crates/myrn-engine/src/api.rs#api::asks_about_history@d16a47875af27ce9
map_version: '2'
---
# api::asks_about_history

Tells whether a task text asks about history, so commits are worth showing.

## Behaviour

- Whole words such as commit, history, regression, broke, changed, reverted or recently give true
- The phrase used to gives true
- Matching is case-insensitive

## Terms

history question, commit intent, regression words, change history

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1332-1357`
- **Signature:** `fn asks_about_history(task: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-context-excluding]]
