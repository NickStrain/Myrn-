---
id: status-parse
type: summary
status: unverified
title: Status::parse
source: myrn summarize
about: sym:crates/myrn-model/src/record.rs#record::Status::parse
anchors:
- sym:crates/myrn-model/src/record.rs#record::Status::parse@690f2edc3e5a1529
map_version: '2'
---
# Status::parse

Looks up a Status by its exact string name, returning None if no variant matches.

## Behaviour

- 'verified' -> Some(Verified)
- 'Verified' or 'VERIFIED' -> None, since matching is case-sensitive
- '' or any unlisted string -> None

## Terms

string to status, from string, lookup by name, exact match, case-sensitive lookup, invalid status

## Connections

- **Location:** `crates/myrn-model/src/record.rs:32-34`
- **Signature:** `pub fn parse(s: &str) -> Option<Status>`
- **In:** [[file-crates-myrn-model-src-record-rs]]
- **Called by:** [[frontmatter-parse]]
