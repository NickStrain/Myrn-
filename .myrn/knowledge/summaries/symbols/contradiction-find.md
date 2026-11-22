---
id: contradiction-find
type: summary
status: unverified
title: contradiction::find
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/contradiction.rs#distill::contradiction::find
anchors:
- sym:crates/myrn-engine/src/distill/contradiction.rs#distill::contradiction::find@1235095b32309ec0
---
# contradiction::find

Takes a connection, a list of target ids in about, and a body. For each target it selects nodes of type fact or decision with an about edge to that target that are not superseded, and returns (id, title) for each node whose doc passes opposite against body. Each node id is checked once, and query errors are propagated.

## Connections

- **Location:** `crates/myrn-engine/src/distill/contradiction.rs:52-76`
- **Signature:** `pub fn find(conn: &Connection, about: &[String], body: &str) -> Result<Vec<(String, String)>>`
- **In:** [[file-crates-myrn-engine-src-distill-contradiction-rs]]
- **Calls:** [[contradiction-opposite]]
- **Called by:** [[engine-record]]
