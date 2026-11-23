---
id: config-contextconfig
type: summary
status: unverified
title: config::ContextConfig
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::ContextConfig
anchors:
- sym:crates/myrn-engine/src/config.rs#config::ContextConfig@52036d2fb3003fa7
map_version: '2'
---
# config::ContextConfig

Controls automatic context injection from hooks, with the token budgets and item and code-size limits for it.

## Behaviour

- inject false -> hooks only capture, nothing is injected
- max_items 5 -> at most 5 items in a prompt pack
- code_lines 40 -> top hit's source shown when its body has 40 lines or fewer

## Terms

context injection, session brief, prompt pack, token budget, hook injection, code snippet

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:163-174`
- **Signature:** `pub struct ContextConfig`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
