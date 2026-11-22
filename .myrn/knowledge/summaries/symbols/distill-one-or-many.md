---
id: distill-one-or-many
type: summary
status: unverified
title: distill::one_or_many
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::one_or_many
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::one_or_many@fce15b4d8e9663cd
---
# distill::one_or_many

Serde deserialize_with helper that maps a missing or null value to an empty Vec, a single string to a one-element Vec, and an array to a Vec unchanged.

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:142-154`
- **Signature:** `fn one_or_many<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error>`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
