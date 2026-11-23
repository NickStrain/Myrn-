---
id: refkind-parse
type: summary
status: unverified
title: RefKind::parse
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::RefKind::parse
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::RefKind::parse@06abcd2e86c23cc2
map_version: '2'
---
# RefKind::parse

Maps a lowercase tag back to its RefKind, returning None for anything else.

## Behaviour

- 'call' -> Some(Call), 'path' -> Some(Path), 'method' -> Some(Method)
- 'Call', '' or 'function' -> None; the match is case-sensitive and exact

## Terms

deserialize, from string, reverse lookup, tag to kind, string to kind, decode

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:46-53`
- **Signature:** `pub fn parse(s: &str) -> Option<RefKind>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[resolver-resolve]]
