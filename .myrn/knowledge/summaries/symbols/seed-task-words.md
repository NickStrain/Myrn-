---
id: seed-task-words
type: summary
status: unverified
title: seed::task_words
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::task_words
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::task_words@9b321f8ce965f541
---
# seed::task_words

Extracts deduplicated search words from task text via `fts::split_identifier`, keeping words of at least 3 characters that are not stopwords and not all digits. Order of first occurrence is preserved.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:136-147`
- **Signature:** `pub fn task_words(task: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Calls:** [[fts-split-identifier]]
- **Called by:** [[seed-seeds]]
- **Tests:** `words_drop_noise`
