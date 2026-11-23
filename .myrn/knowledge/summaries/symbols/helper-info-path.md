---
id: helper-info-path
type: summary
status: unverified
title: helper::info_path
source: myrn summarize
about: sym:crates/myrn-embed/src/helper.rs#helper::info_path
anchors:
- sym:crates/myrn-embed/src/helper.rs#helper::info_path@4614557d1b0e1dc0
map_version: '2'
---
# helper::info_path

Returns the path of helper.json inside the embed subdirectory of data_home. It does not check that the file or directory exists.

## Behaviour

- data_home /x -> /x/embed/helper.json.
- Pure path join; no filesystem access.

## Terms

discovery file location, helper json path, address file path, embed directory

## Connections

- **Location:** `crates/myrn-embed/src/helper.rs:33-35`
- **Signature:** `pub fn info_path(data_home: &Path) -> PathBuf`
- **In:** [[file-crates-myrn-embed-src-helper-rs]]
- **Called by:** [[helper-read-info]], [[helper-serve]]
