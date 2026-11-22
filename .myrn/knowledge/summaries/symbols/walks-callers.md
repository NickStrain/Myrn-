---
id: walks-callers
type: summary
status: unverified
title: walks::callers
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::callers
anchors:
- sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::callers@c3c579cc9de3ad90
---
# walks::callers

Breadth-first walk over incoming `calls` edges from the target ids up to `max_depth`, collecting non-test callers until `cap` is reached (further callers are skipped and set `truncated`) and collecting test nodes separately without expanding them; each node is visited once. Read-only against the DB; errors propagate from the neighbor queries.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs:54-92`
- **Signature:** `pub fn callers( conn: &Connection, targets: &[String], max_depth: usize, cap: usize, ) -> Result<Reach>`
- **In:** [[file-crates-myrn-engine-src-retrieve-walks-rs]]
- **Calls:** [[graph-neighbors]]
- **Called by:** [[engine-impact]]
