---
id: adapter-install
type: summary
status: unverified
title: Adapter::install
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::install
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::install@2e4d4c4de110b07e
---
# Adapter::install

Writes the agent's hooks, MCP, and instruction configuration for the given mode and returns an Installed record of what it changed.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:74`
- **Signature:** `fn install(&self, ctx: &InstallCtx, mode: Mode) -> Result<Installed>;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
