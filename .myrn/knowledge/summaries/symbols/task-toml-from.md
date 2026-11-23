---
id: task-toml-from
type: summary
status: unverified
title: task::toml_from
source: myrn summarize
about: sym:bench/src/task.rs#task::toml_from
anchors:
- sym:bench/src/task.rs#task::toml_from@c63f44eca8a68fb2
map_version: '2'
---
# task::toml_from

Reads a file and deserializes it as TOML into any type T. Parse errors name the file path; read errors do not.

## Behaviour

- A valid TOML file returns Ok with the parsed value.
- Malformed TOML returns an error with the file path added as context.
- A missing or unreadable file returns the raw IO error without the path, because the read step has no context.

## Terms

toml parsing, config file, deserialize, parse error, file read, generic loader

## Connections

- **Location:** `bench/src/task.rs:175-178`
- **Signature:** `fn toml_from<T: serde::de::DeserializeOwned>(p: &Path) -> Result<T>`
- **In:** [[file-bench-src-task-rs]]
- **Called by:** [[task-load-repos]]
