---
id: seed-code-tokens
type: summary
status: unverified
title: seed::code_tokens
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::code_tokens
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::code_tokens@e790f4544ca71647
---
# seed::code_tokens

Splits task text on whitespace and punctuation, trims `.:!?` from the ends, and keeps tokens of at least 3 characters that look like code (contain `_`, `::`, `/`, an inner `.`, or inner camel-case). Private; plain English words are excluded.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:150-169`
- **Signature:** `fn code_tokens(task: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Called by:** [[seed-seeds]]
- **Tests:** `code_like_tokens`
