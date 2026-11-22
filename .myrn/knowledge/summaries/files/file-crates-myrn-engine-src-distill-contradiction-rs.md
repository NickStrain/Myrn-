---
id: file-crates-myrn-engine-src-distill-contradiction-rs
type: summary
status: unverified
title: crates/myrn-engine/src/distill/contradiction.rs
source: myrn summarize
about: file:crates/myrn-engine/src/distill/contradiction.rs
anchors:
- file:crates/myrn-engine/src/distill/contradiction.rs@84959b4a4fcbd6a0
---
# crates/myrn-engine/src/distill/contradiction.rs

Adds contradicts edges between facts and decisions that conflict with a new record, using a narrow lexical check: the statements must share most of their content words and differ in negation parity. It holds the negation word list, the SAME_CLAIM threshold (0.6), the tokenizer, and the opposite, find, and tests items.

## Connections

- **Location:** `crates/myrn-engine/src/distill/contradiction.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[contradiction-words]], [[contradiction-opposite]], [[contradiction-find]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]]
