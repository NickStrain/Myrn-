---
id: file-crates-myrn-src-cmd-verify-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/verify.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/verify.rs
anchors:
- file:crates/myrn/src/cmd/verify.rs@f696a06a75224fea
---
# crates/myrn/src/cmd/verify.rs

Implements the `myrn verify` subcommand, which re-checks stored records against the current code. It either confirms one record by id, or promotes records whose evidence now holds, lists suspect records with their changed or gone anchors, and prints or serializes the result.

## Connections

- **Location:** `crates/myrn/src/cmd/verify.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[verify-run]], [[run-out-verify]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
