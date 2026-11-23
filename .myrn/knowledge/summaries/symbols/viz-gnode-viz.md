---
id: viz-gnode-viz
type: summary
status: unverified
title: viz::gnode
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::gnode
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::gnode@064c59c18052770d
map_version: '2'
---
# viz::gnode

Converts a store Node into a GNode, with a label chosen by type, visibility read from extra JSON, and its group.

## Behaviour

- file or module with an empty path -> title, else id
- commit -> first 7 chars of qname (or id), a space, then the title
- other types -> display_name()

## Terms

label, display name, short hash, node conversion, visibility

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:132-162`
- **Signature:** `fn gnode(n: &Node) -> GNode`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Calls:** [[viz-top-group]], [[visibility-as-str]], [[node-display-name]]
- **Called by:** [[engine-viz-neighborhood]], [[engine-viz-notes-graph]], [[engine-viz-details]]
