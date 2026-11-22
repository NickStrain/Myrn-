---
id: task-toml-from
type: summary
status: unverified
title: task::toml_from
source: myrn summarize
about: sym:bench/src/task.rs#task::toml_from
anchors:
- sym:bench/src/task.rs#task::toml_from@c63f44eca8a68fb2
---
# task::toml_from

Reads a file and deserializes it as TOML into T. Parse errors include the file path via context. Read errors do not.

## Connections

- **Location:** `bench/src/task.rs:175-178`
- **Signature:** `fn toml_from<T: serde::de::DeserializeOwned>(p: &Path) -> Result<T>`
- **In:** [[file-bench-src-task-rs]]
- **Called by:** [[task-load-repos]]
