---
id: signature-words
type: summary
status: unverified
title: Signature::words
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature::words
anchors:
- sym:crates/myrn-engine/src/distill/signature.rs#distill::signature::Signature::words@092a8be8e8201f1f
map_version: '2'
---
# Signature::words

Splits the message into lowercase words for fuzzy matching, dropping tokens shorter than 2 characters and the placeholders N, PATH and ID. The kind is not included.

## Behaviour

- message 'unhashable type: 'list'' -> unhashable, type, list
- message 'Expired(N)' -> expired (N placeholder dropped)
- empty message, or only one-character tokens like 'x 4' -> empty list

## Terms

word list, tokens, keywords, fuzzy match terms, message tokens, placeholder filter

## Connections

- **Location:** `crates/myrn-engine/src/distill/signature.rs:31-37`
- **Signature:** `pub fn words(&self) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Called by:** [[signature-similarity]]
