---
id: file-crates-myrn-src-cmd-init-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/init.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/init.rs
anchors:
- file:crates/myrn/src/cmd/init.rs@11c64beb0b5a5ab8
---
# crates/myrn/src/cmd/init.rs

Implements the `myrn init` subcommand: it indexes the current working directory's repository and creates the local store, covering code only with no session capture. Its single entry point, `run`, delegates indexing to `Engine::init` and formats the resulting report.

## Connections

- **Location:** `crates/myrn/src/cmd/init.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[init-run]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
