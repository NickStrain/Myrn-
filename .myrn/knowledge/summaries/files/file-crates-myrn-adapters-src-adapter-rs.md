---
id: file-crates-myrn-adapters-src-adapter-rs
type: summary
status: unverified
title: crates/myrn-adapters/src/adapter.rs
source: myrn summarize
about: file:crates/myrn-adapters/src/adapter.rs
anchors:
- file:crates/myrn-adapters/src/adapter.rs@1984eab73a7ccda7
---
# crates/myrn-adapters/src/adapter.rs

Defines the Adapter trait that each coding-agent integration implements (detect, install, uninstall, inspect, translate hook payloads, format hook output), plus shared helpers: repo-relative paths, JSON file read/write, hook-context clipping, and a hooks-JSON editor for the nested {"hooks": {Event: [{matcher, hooks: [...]}]}} layout.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs`
- **In:** [[dir-crates-myrn-adapters-src]]
- **Defines:** [[adapter-installctx]], [[adapter-mode]], [[mode-as-str]], [[mode-parse]], [[adapter-change]], [[adapter-installed]], [[adapter-finding]], [[adapter-adapter]], [[adapter-name]], [[adapter-display-name]], [[adapter-detect]], [[adapter-install]], [[adapter-uninstall]], [[adapter-inspect]], [[adapter-translate]], [[adapter-format]], [[adapter-rel]], [[adapter-str-field]], [[adapter-clip]], [[adapter-read-json]], [[adapter-write-json]], [[hooks-json-remove]], [[hooks-json-add]], [[hooks-json-find]]
- **Imports:** [[file-crates-myrn-model-src-event-rs]]
- **Imported by:** [[file-crates-myrn-adapters-src-claude-code-rs]], [[file-crates-myrn-adapters-src-codex-rs]], [[file-crates-myrn-adapters-src-lib-rs]]
