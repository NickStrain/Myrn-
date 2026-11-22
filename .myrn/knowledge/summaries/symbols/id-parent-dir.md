---
id: id-parent-dir
type: summary
status: unverified
title: id::parent_dir
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::parent_dir
anchors:
- sym:crates/myrn-model/src/id.rs#id::parent_dir@6c4f259b89ee917d
---
# id::parent_dir

Takes a repo-relative, forward-slash path and returns the substring before the last `/`, or `""` if there is none; it borrows from the input and does not handle backslashes or a leading `/`.

## Connections

- **Location:** `crates/myrn-model/src/id.rs:42-47`
- **Signature:** `pub fn parent_dir(path: &str) -> &str`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[indexer-persist]], [[resolver-js-import]], [[engine-map-body]]
- **Tests:** `ids`
