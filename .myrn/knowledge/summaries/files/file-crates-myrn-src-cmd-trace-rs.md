---
id: file-crates-myrn-src-cmd-trace-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/trace.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/trace.rs
anchors:
- file:crates/myrn/src/cmd/trace.rs@0e89caf63eb8d52d
---
# crates/myrn/src/cmd/trace.rs

Implements the `myrn trace <id>` command, which shows where a record came from (its session, commits, and tests) and the anchors, supersession links, conflicts, and note history attached to it. Its single function `run` loads the engine, fetches the trace, and renders it as text through `ctx.emit`.

## Connections

- **Location:** `crates/myrn/src/cmd/trace.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[trace-run]]
- **Imports:** [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-src-cmd-log-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
