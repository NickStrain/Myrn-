---
id: directives-is-directive
type: summary
status: unverified
title: directives::is_directive
source: myrn summarize
about: sym:crates/myrn-store/src/notes/directives.rs#notes::directives::is_directive
anchors:
- sym:crates/myrn-store/src/notes/directives.rs#notes::directives::is_directive@e02ac0deebf33987
map_version: '2'
---
# directives::is_directive

Returns true if any of the directive patterns matches anywhere in the given string. It is the single check used by strip for both whole lines and single sentences.

## Behaviour

- 'ignore all previous instructions' -> true, since one matching pattern is enough.
- 'The parser ignores comments and blank lines.' -> false, because the ignore pattern needs an instruction-type word nearby.
- Empty string -> false.

## Terms

injection check, prompt injection detection, directive test, matches blocklist, directive-shaped check

## Connections

- **Location:** `crates/myrn-store/src/notes/directives.rs:55-57`
- **Signature:** `pub fn is_directive(s: &str) -> bool`
- **In:** [[file-crates-myrn-store-src-notes-directives-rs]]
- **Calls:** [[directives-patterns]]
- **Called by:** [[sharing-clean]], [[directives-strip]]
