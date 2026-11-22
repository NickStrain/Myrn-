---
id: copy-dir
type: summary
status: unverified
title: copy_dir
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#copy_dir
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#copy_dir@cf8159461c045252
---
# copy_dir

Recursively copies every entry under from into to, creating directories as needed. Panics on any IO error via unwrap.

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:17-28`
- **Signature:** `fn copy_dir(from: &Path, to: &Path)`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Calls:** [[copy-dir]]
- **Called by:** [[copy-dir]], [[setup]]
