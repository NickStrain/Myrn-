---
id: learn-usage-of
type: summary
status: unverified
title: learn::usage_of
source: myrn summarize
about: sym:crates/myrn-engine/src/learn.rs#learn::usage_of
anchors:
- sym:crates/myrn-engine/src/learn.rs#learn::usage_of@22289eba55500d59
map_version: '2'
---
# learn::usage_of

Runs one cached SQL query per id and returns a map of id to Usage, leaving out ids with no injections in ended sessions. Database errors are returned.

## Behaviour

- empty ids -> empty map
- id never injected, or injected only in sessions with no outcome -> id absent from map
- a used row whose outcome is tests_passed or committed adds to both used and used_good; other outcomes add to used only

## Terms

usage lookup, batch lookup, outcome filter, history query, injection stats fetch, ended sessions only

## Connections

- **Location:** `crates/myrn-engine/src/learn.rs:52-72`
- **Signature:** `pub fn usage_of(conn: &Connection, ids: &[&str]) -> Result<HashMap<String, Usage>>`
- **In:** [[file-crates-myrn-engine-src-learn-rs]]
- **Called by:** [[engine-usage]], [[rank-learned]]
