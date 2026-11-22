---
id: file-crates-myrn-src-cmd-log-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/log.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/log.rs
anchors:
- file:crates/myrn/src/cmd/log.rs@92e7ce83e0fce249
---
# crates/myrn/src/cmd/log.rs

Implements the `myrn log` command, which lists recorded sessions (episodes) or prints one in full, using the engine's episode store and a per-kind event renderer. Decisions and learned records are not included yet; the module doc says they arrive in Phase 2.

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[log-run]], [[run-out]], [[log-render-one]], [[log-datetime]]
- **Imports:** [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
