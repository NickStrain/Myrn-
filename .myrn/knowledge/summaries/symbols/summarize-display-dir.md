---
id: summarize-display-dir
type: summary
status: unverified
title: summarize::display_dir
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::display_dir
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::display_dir@339cfe0652798071
map_version: '2'
---
# summarize::display_dir

Shows the project root's empty path as '.' in error messages; any other path is returned unchanged.

## Behaviour

- '' -> '.'
- 'crates/myrn' -> 'crates/myrn'

## Terms

root label, dot path, project root, error label

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:919-925`
- **Signature:** `fn display_dir(p: &str) -> &str`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Called by:** [[engine-summarize]]
