---
id: status-parse
type: summary
status: unverified
title: Status::parse
source: myrn summarize
about: sym:crates/myrn-model/src/record.rs#record::Status::parse
anchors:
- sym:crates/myrn-model/src/record.rs#record::Status::parse@690f2edc3e5a1529
---
# Status::parse

Looks up a string by linear search over the ALL constant and returns None for unknown input. Matching is case-sensitive, and a new variant that is not added to ALL will never parse, even though as_str handles it.

## Connections

- **Location:** `crates/myrn-model/src/record.rs:32-34`
- **Signature:** `pub fn parse(s: &str) -> Option<Status>`
- **In:** [[file-crates-myrn-model-src-record-rs]]
- **Called by:** [[frontmatter-parse]]
