---
id: recall-recallhit
type: summary
status: unverified
title: recall::RecallHit
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::RecallHit
anchors:
- sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::RecallHit@cdd0345a6cfd499e
map_version: '2'
---
# recall::RecallHit

Result record for one matched bug, holding its id, status, similarity, title, signature, body, root causes, fixing commits, source session and provenance string.

## Behaviour

- causes: each entry is (cause id, safe text, locations), and each location is 'display_name location' from located_in edges
- fixed_in: each entry is the first 7 chars of the commit sha after 'commit:' plus its title
- source is None when the bug's extra JSON has no 'source' key

## Terms

bug hit, match record, root cause list, fixing commit, provenance, similarity score, recall result, source session

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs:19-34`
- **Signature:** `pub struct RecallHit`
- **In:** [[file-crates-myrn-engine-src-retrieve-recall-rs]]
