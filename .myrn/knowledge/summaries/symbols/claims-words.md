---
id: claims-words
type: summary
status: unverified
title: claims::words
source: myrn summarize
about: sym:crates/myrn-store/src/notes/claims.rs#notes::claims::words
anchors:
- sym:crates/myrn-store/src/notes/claims.rs#notes::claims::words@f2c80903d051530e
map_version: '2'
---
# claims::words

Lowercases text, strips apostrophes, and splits on non-alphanumeric characters except underscore. Returns the words longer than 2 characters as a set, plus a count of words found in the NEGATIONS list.

## Behaviour

- 'Never call TokenStore::lookup' -> ({call, tokenstore, lookup}, 1), since 'never' counts as a negation and 'TokenStore::lookup' splits into two words
- "isn't" -> negation count 1, because the apostrophe is removed first so it becomes 'isnt', which is in the list
- 'is a no' -> ({}, 1): 'is' and 'a' are dropped for length 2 or less, and 'no' is checked as a negation before the length filter

## Terms

tokenize, tokenizer, negation count, keyword set, content words, normalize text, apostrophe removal, word extraction

## Connections

- **Location:** `crates/myrn-store/src/notes/claims.rs:17-33`
- **Signature:** `fn words(text: &str) -> (HashSet<String>, usize)`
- **In:** [[file-crates-myrn-store-src-notes-claims-rs]]
- **Called by:** [[claims-opposite]]
