---
id: helper-info
type: summary
status: unverified
title: helper::Info
source: myrn summarize
about: sym:crates/myrn-embed/src/helper.rs#helper::Info
anchors:
- sym:crates/myrn-embed/src/helper.rs#helper::Info@a451eee2848caa07
map_version: '2'
---
# helper::Info

Describes a running helper: its localhost port, access token, process id and embedding dimension. It is stored as JSON in helper.json.

## Behaviour

- Written by serve via info_path and read back by read_info.
- dim is the model's vector length, which clients can use to size buffers.

## Terms

address file, connection info, helper record, port and token, embedding size, discovery record

## Connections

- **Location:** `crates/myrn-embed/src/helper.rs:15-20`
- **Signature:** `pub struct Info`
- **In:** [[file-crates-myrn-embed-src-helper-rs]]
