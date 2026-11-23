---
id: walk-load-ignore
type: summary
status: unverified
title: walk::load_ignore
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/walk.rs#indexer::walk::load_ignore
anchors:
- sym:crates/myrn-engine/src/indexer/walk.rs#indexer::walk::load_ignore@0acba9edfa348c27
map_version: '2'
---
# walk::load_ignore

Builds a Gitignore matcher rooted at root from ignore_file, returning an empty matcher if the file is missing, unreadable, or fails to build.

## Behaviour

- ignore_file does not exist -> empty matcher that ignores nothing
- malformed or unreadable ignore_file -> add error discarded, matcher built from whatever was added
- build failure -> Gitignore::empty(), no error returned

## Terms

ignore file, gitignore, .myrn/ignore, exclude patterns, ignore rules, matcher, skip paths, exclusion list

## Connections

- **Location:** `crates/myrn-engine/src/indexer/walk.rs:23-29`
- **Signature:** `pub fn load_ignore(root: &Path, ignore_file: &Path) -> Gitignore`
- **In:** [[file-crates-myrn-engine-src-indexer-walk-rs]]
- **Called by:** [[engine-capture]], [[walk-walk]]
