---
id: retrieval-render
type: summary
status: unverified
title: retrieval::render
source: myrn summarize
about: sym:bench/src/retrieval.rs#retrieval::render
anchors:
- sym:bench/src/retrieval.rs#retrieval::render@783538ec8f5a4b0c
map_version: '2'
---
# retrieval::render

Formats rows as a markdown table with a Right code column and appends a summary of top-1 hits, in-pack hits, answer terms found, and mean pack tokens.

## Behaviour

- A row with answer terms shows f/n answer terms; otherwise top 1, rank k, or **missed** when hit_rank is None
- Code-task summary counts only rows with terms None; 'in pack' means hit_rank is Some at any rank
- Empty input gives a header, a 0/0 summary, and mean 0 tokens (divisor is max(len, 1))

## Terms

markdown table, summary line, right code, missed task, mean pack tokens, top 1 hit, report output, answer term totals

## Connections

- **Location:** `bench/src/retrieval.rs:102-135`
- **Signature:** `pub fn render(rows: &[Row]) -> String`
- **In:** [[file-bench-src-retrieval-rs]]
- **Called by:** [[real-main]]
