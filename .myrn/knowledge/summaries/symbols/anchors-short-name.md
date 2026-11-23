---
id: anchors-short-name
type: summary
status: unverified
title: anchors::short_name
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/anchors.rs#validity::anchors::short_name
anchors:
- sym:crates/myrn-engine/src/validity/anchors.rs#validity::anchors::short_name@81d8cf319f5a3053
map_version: '2'
---
# anchors::short_name

Returns the part of a symbol id after its last '#', which is the qualified name without the file path, or the whole input if there is no '#'.

## Behaviour

- "sym:src/a.rs#auth::refresh" -> "auth::refresh"
- "auth::refresh" with no '#' -> "auth::refresh" unchanged
- "a#b#c" -> "c" (splits on the last '#')

## Terms

qualified name, symbol suffix, fragment after hash, strip path, item name, display name

## Connections

- **Location:** `crates/myrn-engine/src/validity/anchors.rs:96-98`
- **Signature:** `fn short_name(id: &str) -> &str`
- **In:** [[file-crates-myrn-engine-src-validity-anchors-rs]]
- **Called by:** [[engine-suspect-notice]]
- **Tests:** `anchored_paths`
