---
id: fts-split-identifier
type: summary
status: unverified
title: fts::split_identifier
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::split_identifier
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::split_identifier@a7eede889fdadc50
map_version: '2'
---
# fts::split_identifier

Splits a string into lowercase words at every non-alphanumeric character and at camel-case boundaries, keeping acronyms together.

## Behaviour

- `TokenStore::lookup_by_id` -> [token, store, lookup, by, id]; `HTTPServer` -> [http, server]
- `src/auth/session.rs` -> [src, auth, session, rs]; `v2Api` -> [v2, api]; empty or all-punctuation input -> []
- An uppercase letter starts a new word only after a lowercase letter, a digit, or before a lowercase letter; `HTTP` alone stays one word

## Terms

camel case, snake case, tokenizer, words, lowercase, segment, word boundary, acronym

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:9-34`
- **Signature:** `pub fn split_identifier(s: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Called by:** [[engine-update-vectors]], [[resolver-resolve-method]], [[seed-task-words]], [[seed-seeds]], [[fts-index-node]]
- **Tests:** `splits_identifiers`
