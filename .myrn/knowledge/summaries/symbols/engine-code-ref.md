---
id: engine-code-ref
type: summary
status: unverified
title: Engine::code_ref
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::code_ref
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::Engine::code_ref@4531c2fd7fbb5c66
map_version: '2'
---
# Engine::code_ref

Turns a graph node into a Ref, choosing a header and computing an anchor from its body hash.

## Behaviour

- symbol or test with a unique qname -> header is the qname
- same qname on several symbol or test nodes -> header is path#qname
- symbol or test with body_hash -> anchor target is the header and hash is the first 12 hex chars

## Terms

header, qualified name, body hash, anchor, node reference, qname collision

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:102-133`
- **Signature:** `fn code_ref(&self, n: Node) -> Result<Ref>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Called by:** [[engine-resolve-ref]]
