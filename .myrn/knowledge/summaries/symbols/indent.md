---
id: indent
type: summary
status: unverified
title: indent
source: myrn summarize
about: sym:bench/src/main.rs#indent
anchors:
- sym:bench/src/main.rs#indent@6ea2bdf33210cc46
map_version: '2'
---
# indent

Prefixes every line of a string with four spaces and a trailing newline, for nested display of multi-line notes.

## Behaviour

- 'a\nb' -> ' a\n b\n'
- empty string -> empty string
- input ending in a newline -> no extra indented blank line

## Terms

indentation, indent lines, pad lines, text formatting, nested output, prefix lines

## Connections

- **Location:** `bench/src/main.rs:588-590`
- **Signature:** `fn indent(s: &str) -> String`
- **In:** [[file-bench-src-main-rs]]
- **Called by:** [[real-main]]
