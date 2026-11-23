---
id: distill-one-or-many
type: summary
status: unverified
title: distill::one_or_many
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::one_or_many
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::one_or_many@fce15b4d8e9663cd
map_version: '2'
---
# distill::one_or_many

Serde deserializer helper that reads an optional string or list of strings into a Vec<String>.

## Behaviour

- "a::b" -> ["a::b"].
- ["a","b"] -> ["a","b"]; null -> empty Vec.
- Missing keys are not handled here; the callers use #[serde(default)] for that.

## Terms

string or list, single value, scalar or array, null handling, list coercion

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:142-154`
- **Signature:** `fn one_or_many<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error>`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
