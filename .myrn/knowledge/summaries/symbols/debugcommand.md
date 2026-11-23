---
id: debugcommand
type: summary
status: unverified
title: DebugCommand
source: myrn summarize
about: sym:crates/myrn/src/main.rs#DebugCommand
anchors:
- sym:crates/myrn/src/main.rs#DebugCommand@8f1751a30d599d31
map_version: '2'
---
# DebugCommand

Enum of developer-only subcommands; currently only `review`, which builds a sampled checklist of learned records or scores a marked one.

## Behaviour

- `review` with no flags -> samples 30 records with seed 1, printed to stdout
- `--sample 0` -> all records in the checklist
- `--score FILE` -> reads a marked checklist and reports precision instead of building one

## Terms

precision, gate review, checklist, sampling, seed, record audit

## Connections

- **Location:** `crates/myrn/src/main.rs:219-236`
- **Signature:** `enum DebugCommand`
- **In:** [[file-crates-myrn-src-main-rs]]
