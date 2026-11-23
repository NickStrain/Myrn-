---
id: retrieval-one
type: summary
status: unverified
title: retrieval::one
source: myrn summarize
about: sym:bench/src/retrieval.rs#retrieval::one
anchors:
- sym:bench/src/retrieval.rs#retrieval::one@5ed239ef569d6ee3
map_version: '2'
---
# retrieval::one

Runs myrn context for one task in a fresh scratch home (deleting any existing one), parses the JSON items and text, and computes hit rank and answer-term matches.

## Behaviour

- myrn exits nonzero -> error naming the task id with myrn stderr
- missing or non-array items key -> items 0, hit_rank None, top empty, tokens 0
- answer_contains terms are matched case-insensitively as substrings of the lowercased pack text; Row.text keeps the original case

## Terms

run task, myrn context, pack run, hit rank, answer terms, scratch home, retrieval check, touches match

## Connections

- **Location:** `bench/src/retrieval.rs:35-100`
- **Signature:** `pub fn one( task: &Task, repo: &Repo, checkout: &Path, myrn: &Path, map: Option<&Path>, scratch: &Path, ) -> Result<Row>`
- **In:** [[file-bench-src-retrieval-rs]]
- **Calls:** [[retrieval-item-path]], [[runner-prepare]], [[runner-setup-myrn]], [[kind-as-str]]
- **Called by:** [[real-main]]
