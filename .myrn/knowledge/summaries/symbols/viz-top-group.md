---
id: viz-top-group
type: summary
status: unverified
title: viz::top_group
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::top_group
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::top_group@1c39fb68443759fa
map_version: '2'
---
# viz::top_group

Maps a path to its colour group: the crate name under crates/, otherwise the first folder, otherwise '.'.

## Behaviour

- crates/myrn-engine/src/viz.rs -> myrn-engine
- docs/guide.md -> docs
- README.md, a bare crates path, empty or None -> .

## Terms

colour group, top-level folder, crate name, directory bucket, path prefix

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:122-130`
- **Signature:** `fn top_group(path: Option<&str>) -> String`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Called by:** [[viz-gnode-viz]]
