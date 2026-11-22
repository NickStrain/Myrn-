---
id: dir-crates-myrn-src-cmd
type: summary
status: unverified
title: crates/myrn/src/cmd/
source: myrn summarize
about: module:crates/myrn/src/cmd
anchors:
- module:crates/myrn/src/cmd@45756222e124e57a
---
# crates/myrn/src/cmd/

CLI subcommand handlers for the `myrn` binary; each file implements one `myrn <command>` and is wired through mod.rs, which declares the submodules and defines the shared Ctx (engine access and emit mode) and the thousands formatter. Most handlers build an engine from Ctx, call one engine method, and print text or JSON via ctx.emit; hook.rs is the never-fail entry point agent hook scripts call, and diff, forget, gc, publish, trace, and verify are declared but currently contain only doc comments.

## Connections

- **In:** [[dir-crates-myrn-src]]
- **Files:** [[file-crates-myrn-src-cmd-connect-rs]], [[file-crates-myrn-src-cmd-debug-rs]], [[file-crates-myrn-src-cmd-diff-rs]], [[file-crates-myrn-src-cmd-disconnect-rs]], [[file-crates-myrn-src-cmd-doctor-rs]], [[file-crates-myrn-src-cmd-forget-rs]], [[file-crates-myrn-src-cmd-gc-rs]], [[file-crates-myrn-src-cmd-hook-rs]], [[file-crates-myrn-src-cmd-init-rs]], [[file-crates-myrn-src-cmd-log-rs]], [[file-crates-myrn-src-cmd-mcp-rs]], [[file-crates-myrn-src-cmd-mod-rs]], [[file-crates-myrn-src-cmd-publish-rs]], [[file-crates-myrn-src-cmd-query-rs]], [[file-crates-myrn-src-cmd-rebuild-rs]], [[file-crates-myrn-src-cmd-show-rs]], [[file-crates-myrn-src-cmd-status-rs]], [[file-crates-myrn-src-cmd-summarize-rs]], [[file-crates-myrn-src-cmd-trace-rs]], [[file-crates-myrn-src-cmd-verify-rs]]
