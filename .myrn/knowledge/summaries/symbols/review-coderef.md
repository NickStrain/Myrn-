---
id: review-coderef
type: summary
status: unverified
title: review::CodeRef
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::CodeRef
anchors:
- sym:crates/myrn-engine/src/review.rs#review::CodeRef@f1887cd9d8ce06f3
---
# review::CodeRef

One linked code target: name, location, language, excerpt text, and a changed flag that is true when the recorded anchor hash is not a prefix of the node's current body hash. A node with no body hash also counts as changed when an anchor hash exists.

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:38-45`
- **Signature:** `pub struct CodeRef`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
