---
id: runner-item-path
type: summary
status: unverified
title: runner::item_path
source: myrn summarize
about: sym:bench/src/runner.rs#runner::item_path
anchors:
- sym:bench/src/runner.rs#runner::item_path@e665d5198b9d41fe
---
# runner::item_path

Returns the file path from a node id: the text after the first ':' up to any '#'. For example, sym:src/a.rs#f gives src/a.rs. Returns None when the id has no ':'.

## Connections

- **Location:** `bench/src/runner.rs:540-543`
- **Signature:** `fn item_path(id: &str) -> Option<&str>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-collect-injections]]
- **Tests:** `item_paths`
