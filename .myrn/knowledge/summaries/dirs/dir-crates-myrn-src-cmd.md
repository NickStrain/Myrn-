---
id: dir-crates-myrn-src-cmd
type: summary
status: unverified
title: crates/myrn/src/cmd/
source: myrn summarize
about: module:crates/myrn/src/cmd
anchors:
- module:crates/myrn/src/cmd@55f5562e803fd8c0
---
# crates/myrn/src/cmd/

Command handlers for the `myrn` CLI, one module per subcommand (connect, doctor, diff, forget, gc, hook, init, log, mcp, publish, query, rebuild, show, status, summarize, trace, ui, verify), declared in mod.rs alongside the shared `Ctx` that opens the engine and emits output. Each handler is thin: it parses its arguments, calls one engine method, and renders text or JSON, with the real logic (garbage collection, verification, indexing, knowledge queries) living in the engine and sibling crates.

## Connections

- **In:** [[dir-crates-myrn-src]]
- **Files:** [[file-crates-myrn-src-cmd-connect-rs]], [[file-crates-myrn-src-cmd-debug-rs]], [[file-crates-myrn-src-cmd-diff-rs]], [[file-crates-myrn-src-cmd-disconnect-rs]], [[file-crates-myrn-src-cmd-doctor-rs]], [[file-crates-myrn-src-cmd-embeddings-rs]], [[file-crates-myrn-src-cmd-forget-rs]], [[file-crates-myrn-src-cmd-gc-rs]], [[file-crates-myrn-src-cmd-hook-rs]], [[file-crates-myrn-src-cmd-init-rs]], [[file-crates-myrn-src-cmd-log-rs]], [[file-crates-myrn-src-cmd-mcp-rs]], [[file-crates-myrn-src-cmd-mod-rs]], [[file-crates-myrn-src-cmd-publish-rs]], [[file-crates-myrn-src-cmd-query-rs]], [[file-crates-myrn-src-cmd-rebuild-rs]], [[file-crates-myrn-src-cmd-show-rs]], [[file-crates-myrn-src-cmd-status-rs]], [[file-crates-myrn-src-cmd-summarize-rs]], [[file-crates-myrn-src-cmd-trace-rs]], [[file-crates-myrn-src-cmd-ui-rs]], [[file-crates-myrn-src-cmd-verify-rs]]
