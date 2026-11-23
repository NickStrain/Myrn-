---
id: runner-item-path
type: summary
status: unverified
title: runner::item_path
source: myrn summarize
about: sym:bench/src/runner.rs#runner::item_path
anchors:
- sym:bench/src/runner.rs#runner::item_path@e665d5198b9d41fe
map_version: '2'
---
# runner::item_path

Extracts the file path from a node id: the text between the first : and the first # (or the end).

## Behaviour

- sym:src/a.rs#m::f -> "src/a.rs"
- file:src/a.rs -> "src/a.rs"
- nope, with no colon -> None

## Terms

file path, source file, location, file part, file of symbol

## Connections

- **Location:** `bench/src/runner.rs:695-698`
- **Signature:** `fn item_path(id: &str) -> Option<&str>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-collect-injections]]
- **Tests:** `item_paths`
