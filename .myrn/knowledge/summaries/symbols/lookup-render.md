---
id: lookup-render
type: summary
status: unverified
title: Lookup::render
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Lookup::render
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Lookup::render@9385df959d32616f
---
# Lookup::render

Formats a Lookup as text: Found is passed to the supplied closure, while Ambiguous lists the candidates and NotFound reports no match.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:43-57`
- **Signature:** `pub fn render(&self, f: impl Fn(&T) -> String) -> String`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
