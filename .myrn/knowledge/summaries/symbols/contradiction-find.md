---
id: contradiction-find
type: summary
status: unverified
title: contradiction::find
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/contradiction.rs#distill::contradiction::find
anchors:
- sym:crates/myrn-engine/src/distill/contradiction.rs#distill::contradiction::find@1235095b32309ec0
map_version: '2'
---
# contradiction::find

Returns (id, title) for each fact or decision linked by an `about` edge to any target in `about` whose doc is opposite to `body`, skipping nodes with status 'superseded'. A node linked to several targets is reported once.

## Behaviour

- about = [] -> Ok(empty Vec), no matches
- node linked to two targets with an opposing doc -> listed once, since `seen` is keyed on node id
- NULL title or doc -> treated as '' before the opposite check; NULL status is kept, only the literal 'superseded' is excluded

## Terms

contradiction check, conflicting fact, negated decision, superseded filter, about edge lookup, opposing note, dedupe by id

## Connections

- **Location:** `crates/myrn-engine/src/distill/contradiction.rs:16-40`
- **Signature:** `pub fn find(conn: &Connection, about: &[String], body: &str) -> Result<Vec<(String, String)>>`
- **In:** [[file-crates-myrn-engine-src-distill-contradiction-rs]]
- **Calls:** [[claims-opposite]]
- **Called by:** [[engine-record]]
