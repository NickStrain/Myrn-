---
id: seed-is-question
type: summary
status: unverified
title: seed::is_question
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::is_question
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::is_question@42ada2b47d99924e
map_version: '2'
---
# seed::is_question

Checks whether the first word of the text is a question word or does, is, can or explain. It does not lowercase its input, so the caller must pass lowercase text.

## Behaviour

- how does remeda work -> true
- Does this fail (capital D) -> false, because the check is case sensitive
- fix the bug -> false; empty input -> false

## Terms

question, first word, interrogative, explain request, question word

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:352-368`
- **Signature:** `fn is_question(t: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Called by:** [[seed-asks-about-bug]]
