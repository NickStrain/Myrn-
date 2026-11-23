---
id: retrieval-row
type: summary
status: unverified
title: retrieval::Row
source: myrn summarize
about: sym:bench/src/retrieval.rs#retrieval::Row
anchors:
- sym:bench/src/retrieval.rs#retrieval::Row@e8ec802a69abce5e
map_version: '2'
---
# retrieval::Row

Holds the retrieval result for one task: the 1-based rank of the first hit, answer-term counts for questions, pack size, token count, top item, and raw pack text.

## Behaviour

- hit_rank is None when no pack item path matches a touches file
- terms is None for code tasks (empty answer_contains); Some((found, total)) for questions
- top is an empty string when the pack has no items

## Terms

result record, retrieval result, hit rank, answer terms, pack size, pack tokens, top item

## Connections

- **Location:** `bench/src/retrieval.rs:16-27`
- **Signature:** `pub struct Row`
- **In:** [[file-bench-src-retrieval-rs]]
