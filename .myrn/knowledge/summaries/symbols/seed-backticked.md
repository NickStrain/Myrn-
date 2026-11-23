---
id: seed-backticked
type: summary
status: unverified
title: seed::backticked
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::backticked
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::backticked@9dac95ac3ad8a880
map_version: '2'
---
# seed::backticked

Takes the identifier at the start of each backticked span, such as chunk from chunk([1, 2], 2), keeping only those of 3 or more characters that start with a letter.

## Behaviour

- `chunk([1, 2], 2)` -> chunk
- `GlobSet::new` -> GlobSet::new
- `*.gz` or `ab` -> dropped (starts with a non-letter or is under 3 characters)

## Terms

backtick identifier, inline code name, code literal, function name in backticks

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:191-206`
- **Signature:** `fn backticked(task: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Called by:** [[seed-seeds]]
- **Tests:** `backticked_identifiers`
