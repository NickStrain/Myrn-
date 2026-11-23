---
id: directives-patterns
type: summary
status: unverified
title: directives::patterns
source: myrn summarize
about: sym:crates/myrn-store/src/notes/directives.rs#notes::directives::patterns
anchors:
- sym:crates/myrn-store/src/notes/directives.rs#notes::directives::patterns@079507fb529114f1
map_version: '2'
---
# directives::patterns

Compiles the 14 directive regexes once into a static list and returns that list on every later call. The list covers injection phrases, second-person obligations, role and tool markup, and pipe-to-shell downloads.

## Behaviour

- First call compiles all regexes; later calls return the same slice.
- The <|token|> and [INST] patterns have no case-insensitive flag, so they match only the exact case.
- The role pattern is anchored with ^, so it matches only at the start of the string it is given, which is a line or a sentence when called from strip.

## Terms

directive regexes, injection patterns, compiled rules, blocklist, role markup regex, shell pipe pattern

## Connections

- **Location:** `crates/myrn-store/src/notes/directives.rs:24-52`
- **Signature:** `fn patterns() -> &'static [Regex]`
- **In:** [[file-crates-myrn-store-src-notes-directives-rs]]
- **Called by:** [[directives-is-directive]]
