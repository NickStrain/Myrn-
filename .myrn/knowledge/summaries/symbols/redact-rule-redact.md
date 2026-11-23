---
id: redact-rule-redact
type: summary
status: unverified
title: redact::rule
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::rule
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::rule@31f5a2c34d3c12ab
map_version: '2'
---
# redact::rule

Compiles a regex string into a Rule. It panics if the pattern is invalid.

## Behaviour

- rule('github-token', pattern) -> Rule with kind 'github-token' and the compiled regex
- An invalid pattern panics with 'valid redaction pattern'

## Terms

pattern compilation, regex constructor, rule builder, startup panic

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:24-29`
- **Signature:** `fn rule(kind: &'static str, re: &str) -> Rule`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
