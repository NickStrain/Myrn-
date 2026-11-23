---
id: helper-read-info
type: summary
status: unverified
title: helper::read_info
source: myrn summarize
about: sym:crates/myrn-embed/src/helper.rs#helper::read_info
anchors:
- sym:crates/myrn-embed/src/helper.rs#helper::read_info@713fdadd818682be
map_version: '2'
---
# helper::read_info

Reads and parses helper.json, returning Some(Info) if it exists and is valid, otherwise None.

## Behaviour

- Missing file -> None.
- File with invalid JSON -> None, so errors are never surfaced.
- Valid JSON -> Some(Info).

## Terms

load helper address, check running helper, stale info, discovery, parse address file

## Connections

- **Location:** `crates/myrn-embed/src/helper.rs:37-40`
- **Signature:** `pub fn read_info(data_home: &Path) -> Option<Info>`
- **In:** [[file-crates-myrn-embed-src-helper-rs]]
- **Calls:** [[helper-info-path]]
- **Called by:** [[client-request]], [[helper-serve]]
