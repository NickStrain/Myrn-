---
id: dedupe-merge
type: summary
status: unverified
title: dedupe::merge
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::merge
anchors:
- sym:crates/myrn-engine/src/distill/dedupe.rs#distill::dedupe::merge@bf610c7d3e8a0862
map_version: '2'
---
# dedupe::merge

Updates the old note in place with the new note's evidence, links, anchors, signature, body, title and source, without removing anything from the old note.

## Behaviour

- replace_body false and new body already contained in old body, ignoring whitespace -> body unchanged; otherwise appended after a blank line
- empty old body with replace_body false -> body starts with two newlines
- replace_body true and new title None -> old title kept

## Terms

combine, fold into, update existing, anchor hash, append body, replace text, evidence links

## Connections

- **Location:** `crates/myrn-engine/src/distill/dedupe.rs:124-160`
- **Signature:** `pub fn merge(old: &mut Note, new: &Note, replace_body: bool)`
- **In:** [[file-crates-myrn-engine-src-distill-dedupe-rs]]
- **Called by:** [[engine-record]]
- **Tests:** `merge_combines_and_refreshes_anchors`
