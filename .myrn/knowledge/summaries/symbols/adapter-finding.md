---
id: adapter-finding
type: summary
status: unverified
title: adapter::Finding
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Finding
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Finding@6485adf3a0c07c4f
map_version: '2'
---
# adapter::Finding

Holds the result of one doctor check: an ok flag and a description of the installed piece checked.

## Behaviour

- ok true -> check passed
- ok false -> check failed, what names the piece checked

## Terms

doctor check, health check, status, verification result

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:63-66`
- **Signature:** `pub struct Finding`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
