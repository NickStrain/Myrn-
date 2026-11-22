---
id: visibility-as-str
type: summary
status: unverified
title: Visibility::as_str
source: myrn summarize
about: sym:crates/myrn-model/src/record.rs#record::Visibility::as_str
anchors:
- sym:crates/myrn-model/src/record.rs#record::Visibility::as_str@8f2017fdc460a48f
---
# Visibility::as_str

Returns "private" or "shared" using an exhaustive match. This file has no parse counterpart, so converting a string back to Visibility must happen elsewhere.

## Connections

- **Location:** `crates/myrn-model/src/record.rs:63-68`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-record-rs]]
- **Called by:** [[sync-sync]]
