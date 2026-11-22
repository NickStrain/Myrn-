---
id: file-crates-myrn-src-cmd-query-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/query.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/query.rs
anchors:
- file:crates/myrn/src/cmd/query.rs@ca55399929a4b882
---
# crates/myrn/src/cmd/query.rs

CLI subcommand handlers for the code-graph queries explain, impact, history, context, and brief, each returning the same answer the MCP tools give. Each handler gets an engine from Ctx, runs one query, and passes the result to ctx.emit with a closure that produces the plain-text rendering.

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[query-explain]], [[query-impact]], [[query-history]], [[query-context]], [[query-brief]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
