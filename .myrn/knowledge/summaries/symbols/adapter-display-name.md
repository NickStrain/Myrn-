---
id: adapter-display-name
type: summary
status: unverified
title: Adapter::display_name
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::display_name
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::display_name@539583616c024a92
map_version: '2'
---
# Adapter::display_name

Returns the human-readable agent name used in display output.

## Behaviour

- Returned string is static and set by each implementation
- Not used as an identifier; name() is the id

## Terms

human name, label, pretty name, display label

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:71`
- **Signature:** `fn display_name(&self) -> &'static str;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
