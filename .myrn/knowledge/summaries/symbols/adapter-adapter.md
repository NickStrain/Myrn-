---
id: adapter-adapter
type: summary
status: unverified
title: adapter::Adapter
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter@510e23d408688cd9
map_version: '2'
---
# adapter::Adapter

Is the contract each coding agent integration implements: detect, install, uninstall, inspect, translate hook payloads and format hook output.

## Behaviour

- Trait has no body here; each agent provides its own implementation
- translate returns Ok(None) for events myrn does not use

## Terms

agent integration, plugin interface, agent trait, backend

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:68-85`
- **Signature:** `pub trait Adapter`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
