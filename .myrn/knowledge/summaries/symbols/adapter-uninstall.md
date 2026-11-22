---
id: adapter-uninstall
type: summary
status: unverified
title: Adapter::uninstall
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::uninstall
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::uninstall@cd855ad96b696f63
---
# Adapter::uninstall

Removes everything install wrote; files listed in created are deleted only if myrn's block was their only content. Returns the changes made.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:77`
- **Signature:** `fn uninstall(&self, ctx: &InstallCtx, created: &[String]) -> Result<Vec<Change>>;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
