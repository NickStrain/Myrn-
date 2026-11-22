---
id: walker-note-params
type: summary
status: unverified
title: Walker::note_params
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::note_params
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::note_params@5471687166215b3d
---
# Walker::note_params

Binds parameter names to their declared type names for Rust, Python, and TypeScript parameters, so later method calls on them can resolve.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:299-330`
- **Signature:** `fn note_params(&mut self, def: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-bind-local]]
- **Called by:** [[walker-visit]]
