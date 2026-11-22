---
id: seed-backticked
type: summary
status: unverified
title: seed::backticked
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::backticked
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::backticked@9dac95ac3ad8a880
---
# seed::backticked

Extracts the leading identifier from each backtick-quoted span (chars alphanumeric, `_`, `:`, `.`), trims trailing `.`/`:`, and keeps those of length at least 3 starting with a letter. Lowercase plain words are kept, unlike `code_tokens`.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:173-188`
- **Signature:** `fn backticked(task: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Called by:** [[seed-seeds]]
- **Tests:** `backticked_identifiers`
