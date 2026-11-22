---
id: file-crates-myrn-src-cmd-debug-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/debug.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/debug.rs
anchors:
- file:crates/myrn/src/cmd/debug.rs@34c8d96d8311e109
---
# crates/myrn/src/cmd/debug.rs

Implements the `myrn debug` developer subcommand. `review` either samples learned records into a Gate 2 checklist for a human to mark, or scores a previously marked checklist file, and `render_score` formats the resulting Score as plain text.

## Connections

- **Location:** `crates/myrn/src/cmd/debug.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[debug-review]], [[debug-render-score]]
- **Imports:** [[file-crates-myrn-engine-src-review-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
