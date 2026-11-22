---
id: walks-members
type: summary
status: unverified
title: walks::members
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::members
anchors:
- sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::members@4d6c1702cef9992d
---
# walks::members

Returns the direct members of a struct/enum/class/trait/interface/union/type node by querying symbols whose qname starts with the parent qname plus the language separator ("::" for rust, "." otherwise), then filtering out nested deeper items; returns an empty list for other kinds or nodes lacking path/qname. Results are ordered with same-file symbols first, then by start_line.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs:10-35`
- **Signature:** `pub fn members(conn: &Connection, n: &Node) -> Result<Vec<Node>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-walks-rs]]
- **Calls:** [[graph-nodes-where]]
- **Called by:** [[engine-symbol-card]], [[engine-impact]]
