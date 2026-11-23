---
id: adapter-install
type: summary
status: unverified
title: Adapter::install
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::install
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::install@2e4d4c4de110b07e
map_version: '2'
---
# Adapter::install

Writes the agent's hook and MCP configuration for the given mode and returns a record of what changed.

## Behaviour

- ctx and mode -> Installed listing changes, written, created and manual steps
- Failure -> Err via anyhow; exact conditions are defined by each implementation

## Terms

setup, configure, wire up, enable, hook setup

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:74`
- **Signature:** `fn install(&self, ctx: &InstallCtx, mode: Mode) -> Result<Installed>;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
