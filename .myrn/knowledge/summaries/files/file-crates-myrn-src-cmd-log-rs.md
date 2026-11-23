---
id: file-crates-myrn-src-cmd-log-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/log.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/log.rs
anchors:
- file:crates/myrn/src/cmd/log.rs@915cbd8c00434898
---
# crates/myrn/src/cmd/log.rs

Implements `myrn log`, which lists recorded sessions (episodes) and learned records merged newest first, filtered by type and status, or shows one session in full. It holds the text table renderer, the short type labels, and a UTC timestamp formatter.

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[log-run]], [[run-out]], [[run-item]], [[log-short-type]], [[log-render-one]], [[log-datetime]]
- **Imports:** [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]], [[file-crates-myrn-src-cmd-trace-rs]]
