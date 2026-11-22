---
id: resolver-resolve-sync
type: summary
status: unverified
title: Resolver::resolve
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Resolver::resolve
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Resolver::resolve@7909170dbe8fe72b
---
# Resolver::resolve

Resolves a link target to a node id, trying in order: note id, exact node id, path#qname symbol or test id, commit sha prefix (only when exactly one commit matches), file or module id for a path, then a title match on symbol or test nodes whose qualified name ends with the given segments. The optional hash prefers a same-named symbol whose body hash starts with it. Returns None when nothing matches.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:241-305`
- **Signature:** `fn resolve(&self, t: &str, hash: Option<&str>) -> Result<Option<String>>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[id-file-id]], [[id-module-id]], [[id-symbol-id]], [[id-test-id]], [[graph-nodes-where]], [[sync-segments]], [[sync-looks-like-sha]], [[resolver-note]], [[resolver-exists]]
- **Called by:** [[sync-link]]
