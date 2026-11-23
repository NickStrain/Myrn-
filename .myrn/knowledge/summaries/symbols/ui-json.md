---
id: ui-json
type: summary
status: unverified
title: ui::json
source: myrn summarize
about: sym:crates/myrn/src/cmd/ui.rs#cmd::ui::json
anchors:
- sym:crates/myrn/src/cmd/ui.rs#cmd::ui::json@104f6d5d54d835b8
map_version: '2'
---
# ui::json

Serializes any Serialize value to a compact JSON string, passing serde errors up as Err.

## Behaviour

- Vec of Hit -> compact JSON array string, no pretty printing
- serialization failure -> Err, which serve turns into a 500 response
- used for every /api response body

## Terms

serialize, stringify, encode, to string, json body

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs:186-188`
- **Signature:** `fn json<T: Serialize>(v: &T) -> Result<String>`
- **In:** [[file-crates-myrn-src-cmd-ui-rs]]
- **Called by:** [[ui-serve]]
