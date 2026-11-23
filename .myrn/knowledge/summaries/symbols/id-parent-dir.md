---
id: id-parent-dir
type: summary
status: unverified
title: id::parent_dir
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::parent_dir
anchors:
- sym:crates/myrn-model/src/id.rs#id::parent_dir@6c4f259b89ee917d
map_version: '2'
---
# id::parent_dir

Returns the part of a forward-slash repo path before its last `/`, or an empty string when there is no slash. It borrows from the input and does not allocate.

## Behaviour

- "src/a/b.rs" -> "src/a" (splits at the last slash only)
- "b.rs" -> "" (no slash, so the file is at the root)
- "src/a/" -> "src/a" (trailing slash yields the directory before it)

## Terms

directory, dirname, parent folder, folder of file, containing directory, path prefix

## Connections

- **Location:** `crates/myrn-model/src/id.rs:42-47`
- **Signature:** `pub fn parent_dir(path: &str) -> &str`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[indexer-persist]], [[resolver-js-import]], [[engine-map-body]]
- **Tests:** `ids`
