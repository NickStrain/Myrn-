---
id: contextconfig-default
type: summary
status: unverified
title: ContextConfig::default
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::ContextConfig::default
anchors:
- sym:crates/myrn-engine/src/config.rs#config::ContextConfig::default@6b8e4fb0f11e30ce
map_version: '2'
---
# ContextConfig::default

Supplies the default context settings: injection on, 800 session and 1500 prompt token budgets, 5 items, 40 code lines.

## Behaviour

- no input -> inject true, session_budget 800, prompt_budget 1500
- no input -> max_items 5, code_lines 40

## Terms

context defaults, default token budgets, default injection

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:177-185`
- **Signature:** `fn default() -> Self`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
