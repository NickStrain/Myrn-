---
id: engine-code-ref
type: summary
status: unverified
title: Engine::code_ref
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::code_ref
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::code_ref@4531c2fd7fbb5c66
---
# Engine::code_ref

Converts a graph node into a Ref. For symbol and test nodes the header is the bare qname if it is unique across symbols and tests, otherwise path#qname, and an anchor is attached when the node has a body hash. Other nodes use their path, or their id if they have no path.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:102-133`
- **Signature:** `fn code_ref(&self, n: Node) -> Result<Ref>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Called by:** [[engine-resolve-ref]]
