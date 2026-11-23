---
id: absolute
type: summary
status: unverified
title: absolute
source: myrn summarize
about: sym:bench/src/main.rs#absolute
anchors:
- sym:bench/src/main.rs#absolute@c808a6a9d663bff5
map_version: '2'
---
# absolute

Returns the path unchanged if it is absolute, otherwise joins it onto the current working directory.

## Behaviour

- relative path 'runs/x' -> current directory joined with 'runs/x'
- already absolute path -> returned as given
- path that does not exist -> accepted, nothing is created or checked

## Terms

absolute path, resolve path, relative path, full path, current directory, path join

## Connections

- **Location:** `bench/src/main.rs:580-586`
- **Signature:** `fn absolute(p: &Path) -> Result<PathBuf>`
- **In:** [[file-bench-src-main-rs]]
- **Called by:** [[real-main]]
