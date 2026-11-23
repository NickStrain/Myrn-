---
id: review-reviewitem
type: summary
status: unverified
title: review::ReviewItem
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::ReviewItem
anchors:
- sym:crates/myrn-engine/src/review.rs#review::ReviewItem@547dae452eed8793
map_version: '2'
---
# review::ReviewItem

One exported learned note (id, type, status, title, body, source) with its evidence, typed links, linked code excerpts, and an origin label that the caller fills in.

## Behaviour

- the ntype field is serialized as 'type'
- evidence entries read 'kind: value', or just the value when kind is empty
- inverse links read '<target> fixes this'; others read '<rel> <target>'

## Terms

export record, learned note, evidence, typed link, linked code, origin label, serialized item

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:19-35`
- **Signature:** `pub struct ReviewItem`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
