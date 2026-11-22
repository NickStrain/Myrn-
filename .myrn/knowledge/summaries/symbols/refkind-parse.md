---
id: refkind-parse
type: summary
status: unverified
title: RefKind::parse
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::RefKind::parse
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::RefKind::parse@06abcd2e86c23cc2
---
# RefKind::parse

Inverse of as_str; returns None for any string other than the three known forms.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:46-53`
- **Signature:** `pub fn parse(s: &str) -> Option<RefKind>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[resolver-resolve]]
