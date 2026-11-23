---
id: seed-task-words
type: summary
status: unverified
title: seed::task_words
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::task_words
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::task_words@68823b03095fa5d4
map_version: '2'
---
# seed::task_words

Extracts distinctive search words from a task description, dropping stopwords, all-digit words and repeats.

## Behaviour

- Fix the refresh token bug in the session -> [refresh, token, bug, session]
- two-letter words count only when they appear inside backticks or code tokens, so `*.gz` keeps gz
- one-letter words and two-letter words outside literals are dropped; repeats keep first position

## Terms

search words, keywords, stopwords, task terms, query words, tokens

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:140-156`
- **Signature:** `pub fn task_words(task: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Calls:** [[seed-backticked-spans]], [[seed-code-tokens]], [[fts-split-identifier]]
- **Called by:** [[seed-seeds]]
- **Tests:** `short_words_count_inside_literals`, `words_drop_noise`
