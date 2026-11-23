---
id: adapter-installed
type: summary
status: unverified
title: adapter::Installed
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Installed
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Installed@ac5e8430aef32a8a
map_version: '2'
---
# adapter::Installed

Collects the results of an install: changes shown, files written, instruction files created, and manual steps left to the developer.

## Behaviour

- Default -> all four lists empty
- created -> instruction files that uninstall deletes again if left empty
- manual -> steps the developer must do by hand

## Terms

install result, install report, written files, created files, manual steps

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:51-59`
- **Signature:** `pub struct Installed`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
