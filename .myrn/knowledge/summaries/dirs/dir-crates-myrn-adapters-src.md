---
id: dir-crates-myrn-adapters-src
type: summary
status: unverified
title: crates/myrn-adapters/src/
source: myrn summarize
about: module:crates/myrn-adapters/src
anchors:
- module:crates/myrn-adapters/src@338a628e63171a01
---
# crates/myrn-adapters/src/

Adapter layer for myrn, one module per coding agent (Claude Code, Codex) plus shared code. adapter.rs defines the Adapter trait and shared types (InstallCtx, Mode, Change, Installed, Finding) with JSON/hook-list helpers. lib.rs re-exports these and provides the registry lookups all() and by_name(); detect.rs finds home and executables; pull_only.rs manages the marked context block in instruction files.

## Connections

- **In:** [[dir-crates-myrn-adapters]]
- **Files:** [[file-crates-myrn-adapters-src-adapter-rs]], [[file-crates-myrn-adapters-src-claude-code-rs]], [[file-crates-myrn-adapters-src-codex-rs]], [[file-crates-myrn-adapters-src-detect-rs]], [[file-crates-myrn-adapters-src-lib-rs]], [[file-crates-myrn-adapters-src-pull-only-rs]]
