---
id: refkind-as-str
type: summary
status: unverified
title: RefKind::as_str
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::RefKind::as_str
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::RefKind::as_str@11a13d62b2ec1505
map_version: '2'
---
# RefKind::as_str

Returns the lowercase storage tag for a RefKind.

## Behaviour

- Call -> 'call', Path -> 'path', Method -> 'method'
- Always returns one of the three tags; there is no error case

## Terms

serialize, string tag, label, storage value, to string

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:39-45`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
