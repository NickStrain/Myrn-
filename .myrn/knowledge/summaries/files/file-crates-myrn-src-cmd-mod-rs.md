---
id: file-crates-myrn-src-cmd-mod-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/mod.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/mod.rs
anchors:
- file:crates/myrn/src/cmd/mod.rs@ae9d0cd71d950732
---
# crates/myrn/src/cmd/mod.rs

Module root for the myrn CLI subcommands. It declares one submodule per command, marks diff, forget, gc, publish, trace, and verify as declared but not yet built, and defines the shared Ctx type and the thousands number formatter.

## Connections

- **Location:** `crates/myrn/src/cmd/mod.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[cmd-ctx]], [[ctx-engine]], [[ctx-emit]], [[cmd-thousands]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-src-cmd-connect-rs]], [[file-crates-myrn-src-cmd-debug-rs]], [[file-crates-myrn-src-cmd-diff-rs]], [[file-crates-myrn-src-cmd-disconnect-rs]], [[file-crates-myrn-src-cmd-doctor-rs]], [[file-crates-myrn-src-cmd-forget-rs]], [[file-crates-myrn-src-cmd-gc-rs]], [[file-crates-myrn-src-cmd-hook-rs]], [[file-crates-myrn-src-cmd-init-rs]], [[file-crates-myrn-src-cmd-log-rs]], [[file-crates-myrn-src-cmd-mcp-rs]], [[file-crates-myrn-src-cmd-publish-rs]], [[file-crates-myrn-src-cmd-query-rs]], [[file-crates-myrn-src-cmd-rebuild-rs]], [[file-crates-myrn-src-cmd-show-rs]], [[file-crates-myrn-src-cmd-status-rs]], [[file-crates-myrn-src-cmd-summarize-rs]], [[file-crates-myrn-src-cmd-trace-rs]], [[file-crates-myrn-src-cmd-verify-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-connect-rs]], [[file-crates-myrn-src-cmd-debug-rs]], [[file-crates-myrn-src-cmd-disconnect-rs]], [[file-crates-myrn-src-cmd-doctor-rs]], [[file-crates-myrn-src-cmd-hook-rs]], [[file-crates-myrn-src-cmd-init-rs]], [[file-crates-myrn-src-cmd-log-rs]], [[file-crates-myrn-src-cmd-mcp-rs]], [[file-crates-myrn-src-cmd-query-rs]], [[file-crates-myrn-src-cmd-rebuild-rs]], [[file-crates-myrn-src-cmd-show-rs]], [[file-crates-myrn-src-cmd-status-rs]], [[file-crates-myrn-src-cmd-summarize-rs]], [[file-crates-myrn-src-main-rs]]
