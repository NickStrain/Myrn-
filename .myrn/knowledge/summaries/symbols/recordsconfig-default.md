---
id: recordsconfig-default
type: summary
status: unverified
title: RecordsConfig::default
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::RecordsConfig::default
anchors:
- sym:crates/myrn-engine/src/config.rs#config::RecordsConfig::default@d43c8813472ef14b
map_version: '2'
---
# RecordsConfig::default

Enables both record features by default, with a 500-token failure recall budget.

## Behaviour

- no input -> ask_at_session_end true, recall_on_failure true
- no input -> failure_budget 500

## Terms

default records settings, record defaults

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:127-133`
- **Signature:** `fn default() -> Self`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
