---
id: file-crates-myrn-adapters-src-pull-only-rs
type: summary
status: unverified
title: crates/myrn-adapters/src/pull_only.rs
source: myrn summarize
about: file:crates/myrn-adapters/src/pull_only.rs
anchors:
- file:crates/myrn-adapters/src/pull_only.rs@1799d71f1074fcf3
---
# crates/myrn-adapters/src/pull_only.rs

Pull-only mode for myrn: adds, detects, and removes one marked block (START/END comment markers around the fixed TEXT) in an agent instruction file such as AGENTS.md or CLAUDE.md, telling the agent to call the `context` MCP tool first. Only the marked block is read or changed, and the rest of the file is kept as is.

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs`
- **In:** [[dir-crates-myrn-adapters-src]]
- **Defines:** [[pull-only-block]], [[pull-only-has-block]], [[pull-only-add]], [[pull-only-remove]], [[pull-only-strip]]
- **Imported by:** [[file-crates-myrn-adapters-src-claude-code-rs]], [[file-crates-myrn-adapters-src-codex-rs]], [[file-crates-myrn-adapters-src-lib-rs]]
