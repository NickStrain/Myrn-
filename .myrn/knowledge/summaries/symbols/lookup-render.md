---
id: lookup-render
type: summary
status: unverified
title: Lookup::render
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Lookup::render
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Lookup::render@9385df959d32616f
map_version: '2'
---
# Lookup::render

Formats a lookup as plain text, using the supplied closure for the Found case.

## Behaviour

- Found -> closure output
- Ambiguous -> a header asking for a qualified name or path:line, then one line per candidate with its name and location
- NotFound -> one line saying nothing in the code graph matches the query

## Terms

text output, format result, display, candidate list, error message

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:43-57`
- **Signature:** `pub fn render(&self, f: impl Fn(&T) -> String) -> String`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
