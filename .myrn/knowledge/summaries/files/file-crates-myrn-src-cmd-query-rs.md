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

Command handlers for the query subcommands (explain, impact, history, context, brief) that expose the same engine answers as the MCP tools, each choosing text or structured output via ctx.emit. Each handler builds the engine from Ctx, runs one engine query, and emits the result.

## Connections

- **Location:** `crates/myrn/src/cmd/query.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[query-explain]], [[query-impact]], [[query-history]], [[query-context]], [[query-brief]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
