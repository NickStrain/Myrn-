---
id: walker-in-type-scope
type: summary
status: unverified
title: Walker::in_type_scope
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::in_type_scope
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::in_type_scope@8c95ed79519445bb
---
# Walker::in_type_scope

True when the innermost scope is a Type scope (impl, class, trait, interface).

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:418-420`
- **Signature:** `fn in_type_scope(&self) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-classify-rust]], [[walker-classify-python]]
