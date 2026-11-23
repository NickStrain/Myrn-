---
id: run-out-verify
type: summary
status: unverified
title: run::Out
source: myrn summarize
about: sym:crates/myrn/src/cmd/verify.rs#cmd::verify::run::Out
anchors:
- sym:crates/myrn/src/cmd/verify.rs#cmd::verify::run::Out@c33efa34dcf966c5
map_version: '2'
---
# run::Out

Serializable container for one verify pass, holding the records promoted in that pass and the records currently suspect. It is built in run and used for both JSON output and the text report.

## Behaviour

- built with promote() results first and suspects() results second
- either list may be empty; an empty suspect list produces the 'no suspect records' text
- serialized through ctx.emit, so its field names are the JSON keys verified and suspect

## Terms

verify report, verified list, suspect list, promotions, suspect records, machine output

## Connections

- **Location:** `crates/myrn/src/cmd/verify.rs:24-27`
- **Signature:** `struct Out`
- **In:** [[file-crates-myrn-src-cmd-verify-rs]]
