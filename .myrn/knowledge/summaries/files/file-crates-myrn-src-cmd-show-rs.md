---
id: file-crates-myrn-src-cmd-show-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/show.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/show.rs
anchors:
- file:crates/myrn/src/cmd/show.rs@7917e50ec01e9e66
---
# crates/myrn/src/cmd/show.rs

Implements the `myrn show <id>` subcommand: looks up a single graph node by id via the engine and renders its fields (id, type, kind, name, location, signature, summary, doc, body hash, status, extra) plus its incoming and outgoing relations, as text or structured output depending on the context's emit mode.

## Connections

- **Location:** `crates/myrn/src/cmd/show.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[show-run]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
