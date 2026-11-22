---
id: adapter-format
type: summary
status: unverified
title: Adapter::format
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::format
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::format@743784aa64c6c3a0
---
# Adapter::format

Returns the hook stdout that hands the given context to the model for the given agent event.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:84`
- **Signature:** `fn format(&self, event: &str, context: &str) -> String;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[hook-handle]]
