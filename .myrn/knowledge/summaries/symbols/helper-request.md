---
id: helper-request
type: summary
status: unverified
title: helper::Request
source: myrn summarize
about: sym:crates/myrn-embed/src/helper.rs#helper::Request
anchors:
- sym:crates/myrn-embed/src/helper.rs#helper::Request@f1d12445ae3fa22e
map_version: '2'
---
# helper::Request

One embedding request sent to the helper as a single JSON line, carrying the access token and the texts to encode.

## Behaviour

- Token matches the helper's token -> texts are encoded and a Response is returned.
- Token mismatch -> request is silently ignored with no reply.

## Terms

query, input strings, payload, batch of texts, client message

## Connections

- **Location:** `crates/myrn-embed/src/helper.rs:23-26`
- **Signature:** `pub struct Request`
- **In:** [[file-crates-myrn-embed-src-helper-rs]]
