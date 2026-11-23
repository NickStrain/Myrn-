---
id: one-or-many-oneormany
type: summary
status: unverified
title: one_or_many::OneOrMany
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::one_or_many::OneOrMany
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::one_or_many::OneOrMany@73049a598e3375de
map_version: '2'
---
# one_or_many::OneOrMany

Private untagged enum inside one_or_many that holds either one string or a vector of strings.

## Behaviour

- "x" -> One("x").
- ["x","y"] -> Many(["x","y"]).
- Variants are tried in order: One first, then Many.

## Terms

untagged enum, variant, string or array, deserialize helper

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:145-148`
- **Signature:** `enum OneOrMany`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
