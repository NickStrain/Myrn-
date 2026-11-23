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

Defines the Adapter trait that each coding agent integration implements, the shared types it uses (InstallCtx, Mode, Change, Installed, Finding), and helpers for relative paths, hook context clipping, and reading, writing and editing agent JSON config files and hook lists.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs`
- **In:** [[dir-crates-myrn-adapters-src]]
- **Defines:** [[adapter-installctx]], [[adapter-mode]], [[mode-as-str]], [[mode-parse]], [[adapter-change]], [[adapter-installed]], [[adapter-finding]], [[adapter-adapter]], [[adapter-name]], [[adapter-display-name]], [[adapter-detect]], [[adapter-install]], [[adapter-uninstall]], [[adapter-inspect]], [[adapter-translate]], [[adapter-format]], [[adapter-rel]], [[adapter-str-field]], [[adapter-clip]], [[adapter-read-json]], [[adapter-write-json]], [[hooks-json-remove]], [[hooks-json-add]], [[hooks-json-find]]
- **Imports:** [[file-crates-myrn-model-src-event-rs]]
- **Imported by:** [[file-crates-myrn-adapters-src-claude-code-rs]], [[file-crates-myrn-adapters-src-codex-rs]], [[file-crates-myrn-adapters-src-lib-rs]]
