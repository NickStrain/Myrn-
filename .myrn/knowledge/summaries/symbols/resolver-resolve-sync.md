---
id: resolver-resolve-sync
type: summary
status: unverified
title: Resolver::resolve
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Resolver::resolve
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Resolver::resolve@7909170dbe8fe72b
map_version: '2'
---
# Resolver::resolve

Resolves one link target to a node id by trying, in order: note id, exact node id, path#qname, commit prefix, file or module path, then a qualified-name match on symbol or test titles.

## Behaviour

- '' or ' ' gives None without any lookup
- 'src/x.rs#f' tries the symbol id then the test id and returns None without falling through if neither exists
- a bare name like 'parse' matches symbols whose qname ends with that name; ties prefer a matching hash prefix, then an exact qname, then non-test nodes, then path order

## Terms

link target, target resolution, qualified name, commit prefix, path lookup, symbol lookup, hash disambiguation

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:251-315`
- **Signature:** `fn resolve(&self, t: &str, hash: Option<&str>) -> Result<Option<String>>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[id-file-id]], [[id-module-id]], [[id-symbol-id]], [[id-test-id]], [[graph-nodes-where]], [[sync-segments]], [[sync-looks-like-sha]], [[resolver-note]], [[resolver-exists]]
- **Called by:** [[sync-link]]
