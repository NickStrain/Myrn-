---
id: api-pack
type: summary
status: unverified
title: api::Pack
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Pack
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Pack@31ac91ed5d7cc3a1
map_version: '2'
---
# api::Pack

A rendered context block: its text, the ids of the items included, and its token count.

## Behaviour

- Pack::empty gives empty text, no items and zero tokens
- tokens is computed from the text by budget::tokens

## Terms

context pack, prompt block, injected text, token count, included items

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:77-81`
- **Signature:** `pub struct Pack`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
