---
id: summarize-readme-excerpt
type: summary
status: unverified
title: summarize::readme_excerpt
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::readme_excerpt
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::readme_excerpt@77bcea8fe365134a
map_version: '2'
---
# summarize::readme_excerpt

Reads the first existing file among README.md, README, readme.md, README.rst and README.txt in the root and returns its first 4000 bytes, or None if none exists.

## Behaviour

- text longer than 4000 bytes is cut back to a character boundary
- the first name found wins, even when that file is empty (returns Some with empty text)
- name order decides, so README.md is preferred over README.txt

## Terms

readme, project description, excerpt, repo docs, readme cut

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:886-903`
- **Signature:** `fn readme_excerpt(root: &Path) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Called by:** [[engine-summarize]]
