---
id: adapter-format
type: summary
status: unverified
title: Adapter::format
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::format
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::format@743784aa64c6c3a0
map_version: '2'
---
# Adapter::format

Builds the hook stdout that hands context text to the model for the given agent event.

## Behaviour

- event and context -> string the agent reads as added model context
- Output shape depends on the agent and event name, set by each implementation

## Terms

hook output, stdout, context injection, model context, hook response

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:84`
- **Signature:** `fn format(&self, event: &str, context: &str) -> String;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[hook-handle]]
