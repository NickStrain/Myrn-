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

Manages one marked block in an agent instruction file that tells the agent to call the myrn MCP `context` tool first, with START/END markers and the TEXT body. Public entry points are add (creates or refreshes the block, returns true if the file was created), remove (strips the block and optionally deletes an emptied file it created, returns true if anything changed), and has_block; add and remove leave all other content untouched.

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs`
- **In:** [[dir-crates-myrn-adapters-src]]
- **Defines:** [[pull-only-block]], [[pull-only-has-block]], [[pull-only-add]], [[pull-only-remove]], [[pull-only-strip]]
- **Imported by:** [[file-crates-myrn-adapters-src-claude-code-rs]], [[file-crates-myrn-adapters-src-codex-rs]], [[file-crates-myrn-adapters-src-lib-rs]]
