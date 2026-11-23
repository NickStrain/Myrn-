---
id: dir-crates-myrn-adapters
type: summary
status: unverified
title: crates/myrn-adapters/
source: myrn summarize
about: module:crates/myrn-adapters
anchors:
- module:crates/myrn-adapters@50e9272f984eea43
---
# crates/myrn-adapters/

Adapter layer for myrn, with one module per coding agent (Claude Code, Codex) plus shared code. adapter.rs defines the Adapter trait and shared types (InstallCtx, Mode, Change, Installed, Finding) with JSON and hook-list helpers; lib.rs re-exports them and provides the registry lookups all() and by_name(); detect.rs finds home directories and executables, and pull_only.rs manages the marked context block in instruction files.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-adapters-src]]
