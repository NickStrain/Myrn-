---
id: helper-response
type: summary
status: unverified
title: helper::Response
source: myrn summarize
about: sym:crates/myrn-embed/src/helper.rs#helper::Response
anchors:
- sym:crates/myrn-embed/src/helper.rs#helper::Response@beb86d05e35aafa0
map_version: '2'
---
# helper::Response

The helper's reply, sent as one JSON line: one float vector per input text, in the same order.

## Behaviour

- texts [a, b] -> vectors [vec_a, vec_b] in the same order.
- Written followed by a newline, which is the line delimiter clients read until.

## Terms

embeddings, reply, result vectors, output, float arrays

## Connections

- **Location:** `crates/myrn-embed/src/helper.rs:29-31`
- **Signature:** `pub struct Response`
- **In:** [[file-crates-myrn-embed-src-helper-rs]]
