---
id: adapter-uninstall
type: summary
status: unverified
title: Adapter::uninstall
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::uninstall
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::uninstall@cd855ad96b696f63
map_version: '2'
---
# Adapter::uninstall

Removes everything install wrote, and deletes instruction files in created only when myrn's block was their only content.

## Behaviour

- created empty -> no instruction files deleted
- created file with other user text -> kept, only deleted when nothing but myrn's block was in it
- Returns Vec<Change> describing each removal

## Terms

remove, disconnect, teardown, undo install, cleanup

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:77`
- **Signature:** `fn uninstall(&self, ctx: &InstallCtx, created: &[String]) -> Result<Vec<Change>>;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
